FastComments 提供了一種建立問題並彙總其結果的方法。問題的一個範例（以下稱為 `QuestionConfig`）
可以是星級評分、滑桿，或 NPS 問題（由 `type` 決定）。

問題資料可以單獨彙總、合併彙總、按時間、整體、按頁面等方式彙總。

此框架具備建置客戶端小工具（由您的伺服器置於此 API 之前）、管理儀表板與報表工具所需的所有功能。

首先，我們需要定義一個 `QuestionConfig`。結構如下：

[inline-code-attrs-start title = 'QuestionConfig 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** 只讀 - 每新增一個回應即遞增。 **/
    usedCount: number
    /** 表示設定最後使用（有結果留下）的日期字串。 **/
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


---