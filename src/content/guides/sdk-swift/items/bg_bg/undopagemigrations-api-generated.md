Подрежда пакетна задача, която обръща всяка миграция от една migrate-pages задача, започвайки от най-новата. Налична е в продължение на 90 дни.
Записите, променени след миграцията, не се засягат, така че по-късна редакция никога не се презаписва.
Анкетирайте GET /api/v1/batch-jobs/{batchJobId} за броя на възстановените/пропуснатите.

## Параметри

| Име | Тип | Местоположение | Задължителен | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| undoId | string | path | Да |  |

## Отговор

Връща: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за undoPageMigrations'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let undoId = "undoId_example" // String | 

DefaultAPI.undoPageMigrations(tenantId: tenantId, undoId: undoId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]