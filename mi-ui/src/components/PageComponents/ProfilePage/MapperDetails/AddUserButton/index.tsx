import { FC, useCallback, useRef, useState } from "react";
import { AddInfluenceRequest, addInfluence } from "@services/influence";
import Modal from "@components/SharedComponents/Modal";
import { useGlobalTheme } from "@states/theme";
import { InfluenceTypeEnum, convertFromInfluence } from "@libs/enums";
import InfluenceType from "../../InfluenceList/InfluenceType";

import styles from "./style.module.scss";

type Props = {
  userId: string | number;
  action: "add" | "remove";
  dontShowForm?: boolean;
  onClick?: () => void;
};

const AddUserButton: FC<Props> = ({
  userId,
  action,
  dontShowForm,
  onClick,
}) => {
  const { theme } = useGlobalTheme();
  const [showForm, setShowForm] = useState(false);
  const [description, setDescription] = useState("");
  const [type, setType] = useState<InfluenceTypeEnum>(
    InfluenceTypeEnum.Fascination
  );
  const descriptionRef = useRef<HTMLTextAreaElement>(null);

  const handleClick = useCallback(() => {
    onClick && onClick();
    if (action === "add" && !dontShowForm) {
      setShowForm(true);
    }
  }, [userId, action]);

  const handleSubmit = useCallback(() => {
    const body: AddInfluenceRequest = {
      from_id: Number(userId),
      level: convertFromInfluence(type),
      info: descriptionRef.current?.value || "",
    };

    addInfluence(body).then(() => {
      setShowForm(false);
    });
  }, [userId, type]);

  return (
    <>
      <Modal keepOpen showModal={showForm} setShowModal={setShowForm}>
        <form onSubmit={handleSubmit}>
          <InfluenceType hideRemove editable onChange={setType} />
          <textarea ref={descriptionRef} />
        </form>
      </Modal>
      <button
        className={`${action === "add" ? styles.addUser : styles.removeUser} ${
          theme === "dark" ? styles.dark : styles.light
        }`}
        onClick={handleClick}
      >
        <span>{action === "add" ? "Add" : "Remove"} Influence</span>
      </button>
    </>
  );
};
export default AddUserButton;
