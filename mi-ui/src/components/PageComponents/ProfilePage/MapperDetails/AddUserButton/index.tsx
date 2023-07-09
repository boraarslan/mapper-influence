import Modal from "@components/SharedComponents/Modal";
import { convertFromInfluence,InfluenceTypeEnum } from "@libs/enums";
import {
  AddInfluenceRequest,
  useAddInfluenceMutation,
  useDeleteInfluenceMutation,
} from "@services/influence";
import { useGlobalTheme } from "@states/theme";
import { FC, FormEvent, MouseEvent,useCallback, useState } from "react";

import EditableDescription from "../../EditableDescription";
import InfluenceType from "../../InfluenceList/InfluenceType";
import styles from "./style.module.scss";

type Props = {
  userId: string | number;
  action: "add" | "remove";
  dontShowForm?: boolean;
  onClick?: (e: MouseEvent<HTMLButtonElement>) => void;
};

const AddUserButton: FC<Props> = ({
  userId,
  action,
  dontShowForm,
  onClick,
}) => {
  const { theme } = useGlobalTheme();
  const [loading, setLoading] = useState(false);
  const [showForm, setShowForm] = useState(false);
  const [description, setDescription] = useState("");
  const [type, setType] = useState<InfluenceTypeEnum>(
    InfluenceTypeEnum.Fascination
  );

  const { mutate: addInfluence } = useAddInfluenceMutation();
  const { mutate: deleteInfluence } = useDeleteInfluenceMutation();

  const handleClick = useCallback(
    (e: MouseEvent<HTMLButtonElement>) => {
      onClick && onClick(e); // Used in tutorial
      if (action === "add" && !dontShowForm) {
        setShowForm(true);
      }
      if (action === "remove") {
        setLoading(true);
        deleteInfluence(userId, {
          onSettled: () => setLoading(false),
        });
      }
    },
    [action, dontShowForm, userId, setShowForm, onClick, deleteInfluence]
  );

  const handleSubmit = useCallback(
    (e: FormEvent<HTMLFormElement>) => {
      e.preventDefault();
      setLoading(true);

      const body: AddInfluenceRequest = {
        from_id: Number(userId),
        level: convertFromInfluence(type),
        info: description,
      };

      addInfluence(body, {
        onSuccess: () => setShowForm(false),
        onSettled: () => setLoading(false),
      });
    },
    [userId, type, description, addInfluence]
  );

  const resetForm = () => {
    setDescription("");
    setType(InfluenceTypeEnum.Fascination);
    setShowForm(false);
  };

  return (
    <>
      <Modal
        className={styles.modal}
        keepOpen
        showModal={showForm}
        setShowModal={setShowForm}>
        <form onSubmit={handleSubmit}>
          <InfluenceType
            hideRemove
            editable
            onChange={async (type) => setType(type)}
            className={styles.influenceType}
          />
          <EditableDescription
            description=""
            placeholder="Add a description for your influence."
            editable
            noSubmitOnChange={(e) => setDescription(e.target.value)}
          />
          <div className={styles.buttons}>
            <button type="button" className="cancel" onClick={resetForm}>
              Cancel
            </button>
            <button className="submit">Add</button>
          </div>
        </form>
      </Modal>
      <button
        className={`${
          action === "add" ? styles.addUser : styles.removeUser + " danger"
        } ${theme === "dark" ? styles.dark : styles.light}`}
        disabled={loading}
        onClick={handleClick}>
        <span>{action === "add" ? "Add" : "Remove"} Influence</span>
      </button>
    </>
  );
};
export default AddUserButton;
