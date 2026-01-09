FastCommentsは、質問を構築しその結果を集計する方法を提供します。例としての質問（以下 `QuestionConfig` と呼びます）は、星評価、スライダー、またはNPS質問（`type` により決定）などがあります。

質問データは個別に、まとめて、時間経過で、全体として、ページ別など、さまざまな方法で集計できます。

このフレームワークは、クライアント側ウィジェット（このAPIの前にあなたのサーバーを置く場合）、管理ダッシュボード、レポーティングツールを構築するために必要なすべての機能を備えています。

まず、`QuestionConfig` を定義する必要があります。構造は次のとおりです:

[inline-code-attrs-start title = 'QuestionConfig の構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
type QuestionConfigType = 'nps' | 'slider' | 'star' | 'thumbs';

interface QuestionConfig {
    id: string
    tenantId: string
    name: string
    question: string
    helpText?: string
    createdAt: string
    createdBy: string
    /** 読み取り専用 - 各新しいレスポンスごとにインクリメントされます。 **/
    usedCount: number
    /** 構成が最後に使用された日時の文字列（結果が残されたとき）。 **/
    lastUsed?: string
    type: QuestionConfigType
    numStars?: number
    min?: number
    max?: number
    defaultValue?: number
    labelNegative?: string
    labelPositive?: string
    subQuestionIds?: string[]
    alwaysShowSubQuestions?: boolean
    reportingOrder: number
}
[inline-code-end]