import { FC, useRef, useState } from "react";
import { useOnClickOutside } from "usehooks-ts";
import Arrow from "@components/SvgComponents/Arrow";
import { InfluenceTypeEnum } from "@libs/enums";

import styles from "./style.module.scss";

type Props = {
  className?: string;
  influenceType?: InfluenceTypeEnum;
  editable?: boolean;
  hideRemove?: boolean;
  onChange?: (type: InfluenceTypeEnum) => void;
};

const InfluenceType: FC<Props> = ({
  className,
  editable,
  influenceType = InfluenceTypeEnum.Fascination,
  hideRemove,
  onChange,
}) => {
  const [isOpen, setIsOpen] = useState(false);
  const [selectedType, setSelectedType] = useState(influenceType);

  const ref = useRef(null);
  useOnClickOutside(ref, () => {
    if (isOpen) setIsOpen(false);
  });

  const onRemove = () => {
    // TODO: Remove influence from list
  };

  const handleChange = (type: InfluenceTypeEnum) => {
    setSelectedType(type);
    onChange && onChange(type);
  };

  const dropdownClass = `${styles.dropdown} ${isOpen ? styles.open : ""}`;
  if (editable)
    return (
      <>
        <button
          className={`${dropdownClass} ${className}`}
          onClick={() => setIsOpen((t) => !t)}
          ref={ref}
        >
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
                  disabled={option.label === selectedType}
                >
                  {option.label}
                </button>
              ))}
              {!hideRemove && <button style={{ color: "red" }}>Remove</button>}
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
