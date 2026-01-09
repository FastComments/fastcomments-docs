---
FastComments 提供了一种构建问题并汇总其结果的方式。一个问题的示例（以下称为 `QuestionConfig`）可以是星级评分、滑块，或 NPS 问题（由 `type` 决定）。

问题数据可以按单个、组合、随时间、总体、按页面等方式汇总。

该框架具备构建客户端小部件（在此 API 之前由您的服务器处理）、管理面板和报表工具所需的所有功能。

首先，我们必须定义一个 `QuestionConfig`。其结构如下：

[inline-code-attrs-start title = 'QuestionConfig 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** 只读 - 对每个新响应递增。 **/
    usedCount: number
    /** 表示配置最后一次被使用（留下结果）的日期字符串。 **/
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