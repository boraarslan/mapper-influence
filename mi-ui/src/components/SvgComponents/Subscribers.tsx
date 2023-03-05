import { FC } from "react";

type Props = { color?: string; className?: string };
const Subscribers: FC<Props> = ({ color = "var(--textColor)", className }) => {
  return (
    <svg
      className={className}
      width="35"
      height="45"
      viewBox="0 0 35 45"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M33.3034 28.7814C40.0327 36.6321 25.152 35.5105 16.4803 35.5105C7.80864 35.5105 -4.82858 36.6321 1.90052 28.7814C8.62961 20.9306 2 3 17.5 3C33 3 26.5741 20.9306 33.3034 28.7814Z"
        fill={color}
      />
      <path
        d="M24.4584 37.7292C24.4584 38.6129 24.2843 39.4879 23.9462 40.3043C23.608 41.1208 23.1123 41.8626 22.4875 42.4875C21.8626 43.1123 21.1208 43.608 20.3043 43.9462C19.4879 44.2843 18.6129 44.4584 17.7292 44.4584C16.8455 44.4584 15.9705 44.2843 15.154 43.9462C14.3376 43.608 13.5958 43.1123 12.9709 42.4875C12.3461 41.8626 11.8504 41.1208 11.5122 40.3043C11.1741 39.4879 11 38.6129 11 37.7292L17.7292 37.7292H24.4584Z"
        fill={color}
      />
      <circle cx="17.5" cy="2.5" r="2.5" fill={color} />
    </svg>
  );
};
export default Subscribers;
