import { InfluenceTypeEnum } from "@libs/types/influence";
import { FC, useRef, useState } from "react";
import Arrow from "@components/SvgComponents/Arrow";

import styles from "./style.module.scss";
import { useOnClickOutside } from "usehooks-ts";

type Props = {
  className?: string;
  editable?: boolean;
  influenceType?: InfluenceTypeEnum;
};

const InfluenceType: FC<Props> = ({
  className,
  editable,
  influenceType = InfluenceTypeEnum.Fascination,
}) => {
  const [isOpen, setIsOpen] = useState(false);
  const [selectedType, setSelectedType] = useState(influenceType);

  const ref = useRef(null);
  useOnClickOutside(ref, () => {
    if (isOpen) setIsOpen(false);
  });

  function onRemove() {
    // TODO: Remove influence from list
  }

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
                  onClick={() => setSelectedType(option.label)}
                  disabled={option.label === selectedType}
                >
                  {option.label}
                </button>
              ))}
              <button style={{ color: "red" }}>Remove</button>
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
