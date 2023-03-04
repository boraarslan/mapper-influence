import { FC } from "react";

type Props = { color?: string; className?: string };
const Loved: FC<Props> = ({ color = "var(--textColor)", className }) => {
  return (
    <svg
      className={className}
      width="76"
      height="73"
      viewBox="0 0 76 73"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M38 19C38 29.4934 29.4934 38 19 38C8.50659 38 0 29.4934 0 19C0 8.50659 8.50659 0 19 0C29.4934 0 38 8.50659 38 19Z"
        fill={color}
      />
      <path
        d="M76 19C76 29.4934 67.4934 38 57 38C46.5066 38 38 29.4934 38 19C38 8.50659 46.5066 0 57 0C67.4934 0 76 8.50659 76 19Z"
        fill={color}
      />
      <path d="M38 73L71 32H5L38 73Z" fill={color} />
      <path d="M18 17H61V35H18V17Z" fill={color} />
    </svg>
  );
};
export default Loved;
