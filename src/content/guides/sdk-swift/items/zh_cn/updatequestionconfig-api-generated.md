## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |

## 响应

返回: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/FlagCommentPublic200Response.swift)

## 示例

[inline-code-attrs-start title = 'updateQuestionConfig 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍为测试版。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let updateQuestionConfigBody = UpdateQuestionConfigBody(name: "name_example", question: "question_example", helpText: "helpText_example", type: "type_example", numStars: 123, min: 123, max: 123, defaultValue: 123, labelNegative: "labelNegative_example", labelPositive: "labelPositive_example", customOptions: [QuestionConfig_customOptions_inner(imageSrc: "imageSrc_example", name: "name_example")], subQuestionIds: ["subQuestionIds_example"], alwaysShowSubQuestions: false, reportingOrder: 123) // UpdateQuestionConfigBody | 

DefaultAPI.updateQuestionConfig(tenantId: tenantId, id: id, updateQuestionConfigBody: updateQuestionConfigBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---