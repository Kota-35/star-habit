import { SignupForm } from "./components/SignupForm";
import { SignupHero } from "./components/SignupHero";

const SignupPage = () => {
  return (
    <div className="flex h-screen flex-row">
      <div className="flex w-2/5 flex-col bg-blue-600">
        <SignupHero />
      </div>
      <div className="w-3/5 bg-white">
        <SignupForm />
      </div>
    </div>
  );
};

export default SignupPage;
