import { FC } from "react";

type Props = { color?: string; className?: string };
const Influences: FC<Props> = ({ color = "var(--textColor)", className }) => {
  return (
    <svg
      className={className}
      width="121"
      height="113"
      viewBox="0 0 121 113"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M83 83.9521C77.1605 89.556 69.2325 93 60.5 93C51.7675 93 43.8395 89.556 38 83.9521V113H83V83.9521Z"
        fill={color}
      />
      <path
        fillRule="evenodd"
        clipRule="evenodd"
        d="M38 60.5C38 48.0736 48.0736 38 60.5 38C72.9264 38 83 48.0736 83 60.5C83 72.9264 72.9264 83 60.5 83C48.0736 83 38 72.9264 38 60.5ZM61.712 74C61.2853 74 60.912 73.84 60.592 73.52C60.272 73.2 60.112 72.816 60.112 72.368V52.848L57.776 53.84C57.4133 53.9893 57.072 54.064 56.752 54.064C56.2827 54.064 55.888 53.9147 55.568 53.616C55.2693 53.3173 55.12 52.9333 55.12 52.464C55.12 52.1227 55.2053 51.8133 55.376 51.536C55.5467 51.2587 55.8133 51.056 56.176 50.928L61.168 49.136C61.2747 49.0933 61.3813 49.0613 61.488 49.04C61.5947 49.0187 61.6907 49.008 61.776 49.008C62.2667 49.008 62.6507 49.168 62.928 49.488C63.2267 49.7867 63.376 50.1813 63.376 50.672V72.368C63.376 72.816 63.216 73.2 62.896 73.52C62.576 73.84 62.1813 74 61.712 74Z"
        fill={color}
      />
      <path
        d="M28 87.8876C21.7604 80.4908 18 70.9346 18 60.5C18 37.0279 37.0279 18 60.5 18C83.9721 18 103 37.0279 103 60.5C103 70.9346 99.2396 80.4908 93 87.8876V61H92.9962C92.9987 60.8336 93 60.667 93 60.5C93 42.5507 78.4493 28 60.5 28C42.5507 28 28 42.5507 28 60.5C28 60.667 28.0013 60.8336 28.0038 61H28V87.8876Z"
        fill={color}
      />
      <path
        d="M28 111.538C11.1645 100.796 0 81.9516 0 60.5C0 27.0868 27.0868 0 60.5 0C93.9132 0 121 27.0868 121 60.5C121 81.9516 109.835 100.796 93 111.538V100.452C104.594 91.0088 112 76.6192 112 60.5C112 32.0573 88.9427 9 60.5 9C32.0573 9 9 32.0573 9 60.5C9 76.6192 16.4056 91.0088 28 100.452V111.538Z"
        fill={color}
      />
    </svg>
  );
};
export default Influences;
