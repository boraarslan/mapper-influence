import { FC, useRef, useState } from "react";
import { useOnClickOutside } from "usehooks-ts";
import { toast } from "react-toastify";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import Arrow from "@components/SvgComponents/Arrow";
import { InfluenceTypeEnum, convertToInfluence } from "@libs/enums";
import Modal from "@components/SharedComponents/Modal";
import {
  InfluenceResponse,
  deleteInfluence,
  getInfluences,
} from "@services/influence";

import styles from "./style.module.scss";
import { useCurrentUser } from "@hooks/useUser";

type Props = {
  className?: string;
  editable?: boolean;
  influenceData?: InfluenceResponse;
  hideRemove?: boolean;
  onChange?: (type: InfluenceTypeEnum) => Promise<any>;
  noSubmitOnChange?: (type: InfluenceTypeEnum) => void;
};

const InfluenceType: FC<Props> = ({
  className,
  editable,
  influenceData,
  hideRemove,
  onChange,
  noSubmitOnChange,
}) => {
  const [isLoading, setIsLoading] = useState(false);
  const [isOpen, setIsOpen] = useState(false);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [selectedType, setSelectedType] = useState<InfluenceTypeEnum>(
    convertToInfluence(influenceData?.influence_level || 1)
  );

  const { user } = useCurrentUser();
  const queryClient = useQueryClient();

  const ref = useRef(null);
  useOnClickOutside(ref, () => {
    if (isOpen) setIsOpen(false);
  });

  const onRemove = () => {
    setIsLoading(true);
    return deleteInfluence(influenceData?.from_id || 0).finally(() => {
      setIsLoading(false);
      setIsModalOpen(false);
    });
  };

  const updateInfluences = useMutation(onRemove, {
    onMutate: async () => {
      await queryClient.cancelQueries(["influences", user?.id]);
      const previousInfluences = queryClient.getQueryData<InfluenceResponse[]>([
        "influences",
        user?.id,
      ]);
      if (previousInfluences) {
        queryClient.setQueryData<InfluenceResponse[]>(
          ["influences", user?.id],
          previousInfluences.filter(
            (influence) => influence.from_id !== influenceData?.from_id
          )
        );
      }
      return { previousInfluences };
    },
    onError: (err, _, context: any) => {
      if (context?.previousInfluences) {
        queryClient.setQueryData<InfluenceResponse[]>(
          ["influences", user?.id],
          context.previousInfluences
        );
      }
      toast.error("Failed to delete influence.");
    },
    onSuccess: () => toast.success("Influence deleted."),
    onSettled: () => queryClient.invalidateQueries(["influences", user?.id]),
  });

  const handleChange = (newType: InfluenceTypeEnum) => {
    setSelectedType(newType);
    noSubmitOnChange && noSubmitOnChange(newType);
    if (onChange) {
      setIsLoading(true);
      onChange(newType)
        .catch(() => {
          setSelectedType(selectedType);
          toast.error("Failed to update influence level.");
        })
        .finally(() => setIsLoading(false));
    }
  };

  const dropdownClass = `${styles.dropdown} ${isOpen ? styles.open : ""} ${
    isLoading ? styles.disabled : ""
  }`;
  if (editable)
    return (
      <>
        <Modal
          setShowModal={setIsModalOpen}
          showModal={isModalOpen}
          className={`${styles.modal}`}>
          <h4>Are you sure you want to delete this influence?</h4>
          <div>
            <button
              className="cancel"
              disabled={isLoading}
              onClick={() => setIsModalOpen(false)}>
              Cancel
            </button>
            <button
              className="danger"
              disabled={isLoading}
              onClick={() => updateInfluences.mutate()}>
              Delete
            </button>
          </div>
        </Modal>
        <div
          className={`${dropdownClass} ${className}`}
          ref={ref}
          onClick={() => !isLoading && setIsOpen((t) => !t)}>
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
        </div>
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
