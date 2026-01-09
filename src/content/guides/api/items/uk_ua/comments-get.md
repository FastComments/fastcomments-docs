[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Цей API використовується для отримання коментарів для відображення користувачу. Наприклад, він автоматично відфільтровує непідтверджені або спам-коментарі.

### Pagination

Пагінація може здійснюватися одним із двох способів, залежно від вимог до продуктивності та випадку використання:

1. Найшвидший: **Попередньо обчислена пагінація**:
   1. Так працює FastComments, коли ви використовуєте наші готові віджети та клієнти.
   2. Натискання "next" просто збільшує номер сторінки.
   3. Це можна уявити як отримання з key-value сховища.
   4. У цьому випадку просто визначте параметр `page`, що починається з `0`, та напрям сортування як `direction`.
   5. Розміри сторінок можна налаштовувати через правила кастомізації.
2. Найгнучкіший: **Гнучка пагінація**:
   1. У цьому випадку ви можете визначити кастомні параметри `limit` і `skip`. Не передавайте `page`.
   2. Підтримується також напрям сортування `direction`.
   3. `limit` — це загальна кількість для повернення після застосування `skip`.
      - Приклад: встановіть `skip = 200, limit = 100`, коли `page size = 100` і `page = 2`.
   4. Дочірні коментарі все одно враховуються в пагінації. Ви можете обійти це, використавши опцію `asTree`.
      - Ви можете здійснювати пагінацію дітей через `limitChildren` і `skipChildren`.
      - Ви можете обмежити глибину повернутих тредів за допомогою `maxTreeDepth`.

### Threads

1. При використанні `Precalculated Pagination` коментарі групуються за *сторінкою*, і коментарі в тредах впливають на загальну сторінку.
   1. У цьому випадку треди можна визначити на клієнті на основі `parentId`.
   2. Наприклад, на сторінці з одним коментарем верхнього рівня та 29 відповідями, і встановивши `page=0` в API — ви отримаєте тільки верхній рівень та 29 дочірніх.
   3. [Приклад зображення, що ілюструє кілька сторінок.](https://blog.winricklabs.com/images/fc-pagination02.png)
2. При використанні `Flexible Pagination` ви можете визначити параметр `parentId`.
   1. Встановіть його в null, щоб отримати лише коментарі верхнього рівня.
   2. Потім, щоб переглянути треди, викличте API ще раз і передайте `parentId`.
   3. Звичне рішення — зробити виклик API для коментарів верхнього рівня, а потім паралельні виклики API, щоб отримати коментарі для дітей кожного коментаря.
3. __НОВИНКА станом на лютий 2023!__ Отримуйте у вигляді дерева, використовуючи `&asTree=true`.
   1. Це можна уявити як `Гнучка пагінація у вигляді дерева`.
   2. У пагінації враховуються тільки коментарі верхнього рівня.
   3. Встановіть `parentId=null`, щоб почати дерево з кореня (ви повинні встановити `parentId`).
   4. Встановіть `skip` і `limit` для пагінації.
   5. Встановіть `asTree` в `true`.
   6. Вартість в кредитах збільшується в `2x`, оскільки наш бекенд повинен виконати набагато більше роботи в цьому сценарії.
   7. Встановіть `maxTreeDepth`, `limitChildren` і `skipChildren` за потреби.

### Trees Explained

Коли використовується `asTree`, зрозуміти пагінацію може бути складно. Ось корисна схема:

<div class="screenshot white-bg">
    <div class="title">Діаграма пагінації дерева</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Діаграма пагінації дерева" />
</div>

### Fetching Comments in The Context of a User

API `/comments` може використовуватися в двох контекстах, для різних випадків використання:

- Для повернення коментарів, відсортованих та позначених інформацією для побудови власного клієнта.
  - У цьому випадку визначте параметр запиту `contextUserId`.
- Для отримання коментарів з вашого бекенду для кастомних інтеграцій.
  - Платформа за замовчуванням використовуватиме цей варіант без `contextUserId`. 

[inline-code-attrs-start title = 'Попередньо обчислена пагінація коментарів'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Гнучка пагінація коментарів'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Гнучка пагінація коментарів у контексті користувача'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Гнучка пагінація коментарів у контексті користувача (тільки верхній рівень)'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Get Comments as a Tree

Можна отримати коментарі у вигляді дерева, причому в пагінації враховуються лише коментарі верхнього рівня.

[inline-code-attrs-start title = 'Коментарі у вигляді дерева в контексті користувача'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Хочете отримати лише коментарі верхнього рівня та їхніх безпосередніх дочірніх? Ось один зі способів:

[inline-code-attrs-start title = 'Коментарі у вигляді дерева з максимальною глибиною'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Однак у вашому інтерфейсі може знадобитися знати, чи показувати кнопку «show replies» під кожним коментарем. Під час отримання коментарів у вигляді дерева до коментарів додається властивість `hasChildren`, коли це застосовно.

### Get Comments as a Tree, Searching by Hash Tag

Можна шукати за хештегом через API по всьому вашому орендарю (не обмежуючись однією сторінкою або `urlId`).

У цьому прикладі ми опускаємо `urlId` і шукаємо за кількома хештегами. API поверне лише ті коментарі, які містять усі запитані хештеги.

[inline-code-attrs-start title = 'Коментарі у вигляді дерева в контексті користувача, за хештегом'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### All Request Params

[inline-code-attrs-start title = 'Структура запиту коментарів'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** urlId (URL сторінки або ідентифікатор статті), з якою пов'язані коментарі. **/
    urlId?: string
    /** Обмежити коментарі, повернені цим користувачем. **/
    userId?: string
    /** Використовуйте це для пошуку за хештегом. Щоб отримати перетин кількох хештегів, робіть &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** Напрям сортування. За замовчуванням MR (Найбільш релевантні). Інші опції: OF (Спочатку найстаріші) та NF (Спочатку найновіші). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Попередньо обчислена пагінація: сторінка для отримання, починаючи з 0. Передайте -1 для всіх коментарів (до 250). **/
    page?: number
    /** Гнучка пагінація: скільки коментарів ми повинні повернути? **/
    limit?: number
    /** Гнучка пагінація: скільки дочірніх коментарів ми повинні повернути для кожного батька? **/
    limitChildren?: number
    /** Гнучка пагінація: скільки коментарів ми повинні пропустити? **/
    skip?: number
    /** Гнучка пагінація: скільки дочірніх коментарів ми повинні пропустити для кожного батька? **/
    skipChildren?: number
    /** Для визначення заблокованих та позначених коментарів. **/
    contextUserId?: string
    /** Для визначення заблокованих та позначених коментарів. **/
    anonUserId?: string
    /** Для отримання дочірніх коментарів. **/
    parentId?: string
    /** Для отримання у вигляді дерева. **/
    asTree?: boolean
    /** Наскільки глибоко по дереву ми повинні повертати дані? 0 повертає жодних дітей. 1 повертає безпосередніх дітей тощо. **/
    maxTreeDepth?: number
}
[inline-code-end]

### The Response

[inline-code-attrs-start title = 'Структура відповіді коментарів'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Додається у разі збою. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Додається у разі збою. **/
    reason?: string
    /** Коментарі! **/
    comments: Comment[]
}
[inline-code-end]

### Helpful Tips

#### URL ID

Ймовірно, ви захочете використовувати API `Comment` з параметром `urlId`. Ви можете спочатку викликати API `Pages`, щоб побачити, як виглядають доступні для вас значення `urlId`. 

#### Anonymous Actions

Для анонімного коментування ймовірно варто передавати `anonUserId` під час отримання коментарів, а також під час виконання флагування та блокування.

(!) Це обов'язково для багатьох магазинів застосунків, оскільки користувачі повинні мати можливість позначати контент, створений користувачами, який вони бачать, навіть якщо вони не увійшли в систему. Невиконання цього може призвести до видалення вашого застосунку з відповідного магазину.

#### Comments Not Being Returned

Перевірте, що ваші коментарі схвалені і не є спамом.

---