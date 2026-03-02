import { Check, X } from "lucide-react";
import { match } from "ts-pattern";
import { usePasswordStrength } from "./_.hook";

interface Props {
  password: string;
  isVisible: boolean;
}

export const PasswordStrength = (props: Props) => {
  const {
    isPasswordAtLeast8Chars,
    isPasswordContainingDigit,
    shouldShowStrength,
  } = usePasswordStrength(props);

  if (!shouldShowStrength) return null;
  return (
    <div>
      <div className="flex items-center space-x-2">
        {match(isPasswordAtLeast8Chars)
          .with(true, () => <Check className="h-3 w-3 text-green-500" />)
          .with(false, () => <X className="h-3 w-3 text-red-500" />)
          .exhaustive()}
        <span className="text-xs">8文字以上</span>
      </div>
      <div className="flex items-center space-x-2">
        {match(isPasswordContainingDigit)
          .with(true, () => <Check className="h-3 w-3 text-green-500" />)
          .with(false, () => <X className="h-3 w-3 text-red-500" />)
          .exhaustive()}
        <span className="text-xs">数字を含む</span>
      </div>
    </div>
  );
};
