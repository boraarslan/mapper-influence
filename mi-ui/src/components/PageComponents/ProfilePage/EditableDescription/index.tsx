import { ChangeEvent, ChangeEventHandler, FC } from "react";
import AwesomeDebouncePromise from "awesome-debounce-promise";

import styles from "./style.module.scss";

type Props = {
  className?: string;
  description: string;
  placeholder: string;
  label: string;
  editable?: boolean;
  onChange?: ChangeEventHandler<HTMLTextAreaElement>;
};
const EditableDescription: FC<Props> = ({
  className,
  description,
  editable,
  label,
  placeholder,
  onChange,
}) => {
  const debouncedSubmit = AwesomeDebouncePromise(
    (e: ChangeEvent<HTMLTextAreaElement>) => {
      onChange && onChange(e);
    },
    500
  );

  return (
    <>
      <textarea
        aria-label={label}
        className={`${className} ${styles.description} ${
          editable ? styles.editable : ""
        }`}
        onChange={(e) => debouncedSubmit(e)}
        defaultValue={description}
        placeholder={editable ? placeholder : ""}
        readOnly={!editable}
        disabled={!editable}
      />
    </>
  );
};
export default EditableDescription;
