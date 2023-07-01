import { ChangeEvent, ChangeEventHandler, FC } from "react";
import AwesomeDebouncePromise from "awesome-debounce-promise";

import styles from "./style.module.scss";
import { toast } from "react-toastify";

type Props = {
  className?: string;
  description: string;
  placeholder: string;
  label: string;
  editable?: boolean;
  statusText?: {
    loading?: string;
    success?: string;
    error?: string;
  };
  onChange?: (e: ChangeEvent<HTMLTextAreaElement>) => Promise<any>;
};
const EditableDescription: FC<Props> = ({
  className,
  description,
  editable,
  label,
  placeholder,
  statusText = {
    error: "Could not submit.",
    success: "Successfully submitted.",
    loading: "Submitting.",
  },
  onChange,
}) => {
  const debouncedSubmit = AwesomeDebouncePromise(
    (e: ChangeEvent<HTMLTextAreaElement>) => {
      if (onChange) {
        const loadingToast = toast.loading(statusText?.loading);
        onChange(e)
          .then(() => {
            toast.update(loadingToast, {
              render: statusText?.success,
              type: toast.TYPE.SUCCESS,
              isLoading: false,
              autoClose: 5000,
            });
          })
          .catch(() =>
            toast.update(loadingToast, {
              render: statusText?.error,
              type: toast.TYPE.ERROR,
              isLoading: false,
              autoClose: 5000,
            })
          );
      }
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
