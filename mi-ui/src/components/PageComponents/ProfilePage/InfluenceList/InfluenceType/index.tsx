import { FC, useRef, useState } from "react";
import { useOnClickOutside } from "usehooks-ts";
import { toast } from "react-toastify";
import { useQueryClient } from "@tanstack/react-query";
import Arrow from "@components/SvgComponents/Arrow";
import { InfluenceTypeEnum, convertToInfluence } from "@libs/enums";
import Modal from "@components/SharedComponents/Modal";
import { InfluenceResponse, deleteInfluence } from "@services/influence";

import styles from "./style.module.scss";

type Props = {
  className?: string;
  editable?: boolean;
  influenceData: InfluenceResponse;
  hideRemove?: boolean;
  onChange?: (type: InfluenceTypeEnum) => Promise<any>;
};

const InfluenceType: FC<Props> = ({
  className,
  editable,
  influenceData,
  hideRemove,
  onChange,
}) => {
  const [isLoading, setIsLoading] = useState(false);
  const [isOpen, setIsOpen] = useState(false);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [selectedType, setSelectedType] = useState<InfluenceTypeEnum>(
    convertToInfluence(influenceData.influence_level || 1)
  );

  const queryClient = useQueryClient();

  const ref = useRef(null);
  useOnClickOutside(ref, () => {
    if (isOpen) setIsOpen(false);
  });

  const onRemove = () =>
    deleteInfluence(influenceData.from_id)
      .then(() => {
        queryClient.invalidateQueries(["influences", influenceData.to_id], {
          exact: true,
        });
      })
      .finally(() => setIsModalOpen(false));

  const handleChange = (newType: InfluenceTypeEnum) => {
    if (onChange) {
      setIsLoading(true);
      setSelectedType(newType);
      onChange(newType)
        .catch(() => {
          setSelectedType(selectedType);
          toast.error("Failed to update influence level.");
        })
        .finally(() => setIsLoading(false));
    }
  };

  const dropdownClass = `${styles.dropdown} ${isOpen ? styles.open : ""}`;
  if (editable)
    return (
      <>
        <Modal
          setShowModal={setIsModalOpen}
          showModal={isModalOpen}
          className={styles.modal}>
          <h4>Are you sure you want to delete this influence?</h4>
          <div>
            <button className="cancel" onClick={() => setIsModalOpen(false)}>
              Cancel
            </button>
            <button className="danger" onClick={onRemove}>
              Delete
            </button>
          </div>
        </Modal>
        <button
          className={`${dropdownClass} ${className}`}
          ref={ref}
          disabled={isLoading}
          onClick={() => setIsOpen(true)}>
          <span>
            {selectedType}{" "}
            <Arrow className={styles.arrow} color="var(--textColor)" />
          </span>

          {isOpen && (
            <div className={styles.optionsCont}>
              {DROPDOWN_OPTIONS.map((option) => (
                <button
                  key={option.value}
                  onClick={() => handleChange(option.label)}
                  disabled={option.label === selectedType}>
                  {option.label}
                </button>
              ))}
              {!hideRemove && (
                <button
                  style={{ color: "red" }}
                  onClick={() => {
                    setIsOpen(false);
                    setIsModalOpen(true);
                  }}>
                  Remove
                </button>
              )}
            </div>
          )}
        </button>
      </>
    );

  return (
    <div className={styles.nonEditable}>
      <InfluenceText type={selectedType} />
    </div>
  );
};
export default InfluenceType;

const DROPDOWN_OPTIONS = [
  { label: InfluenceTypeEnum.Respect, value: 1 },
  { label: InfluenceTypeEnum.Fascination, value: 4 },
  { label: InfluenceTypeEnum.Implementation, value: 7 },
];

const InfluenceText: FC<{ type: InfluenceTypeEnum }> = ({ type }) => {
  switch (type) {
    case InfluenceTypeEnum.Respect:
      return <>Respects</>;
    case InfluenceTypeEnum.Fascination:
      return (
        <>
          Fascinated <span>by</span>
        </>
      );
    case InfluenceTypeEnum.Implementation:
      return (
        <>
          Implements <span>from</span>
        </>
      );
    default:
      return <>Respect</>;
  }
};
