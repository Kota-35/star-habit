import {
  Activity,
  ChartNoAxesCombined,
  LayoutDashboardIcon,
  Star,
} from "lucide-react";

export const SignupHero = () => {
  return (
    <div className="flex h-full flex-col justify-between px-10 py-12 text-white">
      {/* ヘッダー: ロゴ + アプリ名 */}
      <header className="flex items-center gap-2">
        <Star className="size-8" />
        <span className="font-semibold text-lg">STAR法習慣化</span>
      </header>

      {/* メインコピー + 概要 */}
      <div className="space-y-4">
        <h1 className="font-bold text-3xl leading-tight">
          STAR法で、
          <br />
          あなたの経験を
          <br />
          資産に変える
        </h1>
        <p className="max-w-md text-sm text-white/95 leading-relaxed">
          毎日の振り返りを構造化し、キャリアの成長を可視化しましょう。
          <br />
          数千人のプロフェッショナルが習慣化に成功しています。
        </p>
      </div>

      {/* 機能リスト */}
      <ul className="mb-10 space-y-6">
        <FeatureItem
          icon={<LayoutDashboardIcon />}
          title="かんたん記録"
          description="S・T・A・Rの4ステップで、誰でも論理的な振り返りが可能に。"
        />
        <FeatureItem
          icon={<ChartNoAxesCombined />}
          title="成長の可視化"
          description="ヒートマップやAI分析で、あなたの努力と成果を一目で確認。"
        />
        <FeatureItem
          icon={<Activity />}
          title="AIコーチング"
          description="あなたのログに基づいて、次のアクションをAIがアドバイス。"
        />
      </ul>
    </div>
  );
};

const FeatureItem = ({
  icon,
  title,
  description,
}: {
  icon: React.ReactNode;
  title: string;
  description: string;
}) => (
  <li className="flex gap-4">
    <div className="flex size-10 shrink-0 items-center justify-center rounded-lg bg-white/15">
      {icon}
    </div>
    <div className="space-y-1">
      <h3 className="font-bold">{title}</h3>
      <p className="text-sm text-white/95 leading-relaxed">{description}</p>
    </div>
  </li>
);
