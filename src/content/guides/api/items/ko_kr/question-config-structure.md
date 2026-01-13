FastComments는 질문을 구성하고 그 결과를 집계하는 방법을 제공합니다. 질문의 예(이하 `QuestionConfig`라고 함)는 별점, 슬라이더, 또는 NPS 질문(`type`에 의해 결정될 수 있음)이 될 수 있습니다.

질문 데이터는 개별적으로, 함께, 시간 경과에 따라, 전체적으로, 페이지별로 등으로 집계할 수 있습니다.

이 프레임워크는 클라이언트 측 위젯(이 API 앞에 서버를 두는 방식), 관리자 대시보드 및 리포팅 도구를 구축하는 데 필요한 모든 기능을 제공합니다.

먼저 `QuestionConfig`를 정의해야 합니다. 구조는 다음과 같습니다:

[inline-code-attrs-start title = 'QuestionConfig 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** 읽기 전용 - 각 새 응답마다 증가합니다. **/
    usedCount: number
    /** 구성이 마지막으로 사용(결과가 남겨진)된 시점의 날짜 문자열입니다. **/
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