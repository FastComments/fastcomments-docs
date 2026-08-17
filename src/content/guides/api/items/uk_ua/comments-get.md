[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Цей API використовується для отримання коментарів для відображення користувачеві. Наприклад, він автоматично фільтрує непідтверджені або спам‑коментарі.

### Пагінація

Пагінація може здійснюватися одним із двох способів, залежно від вимог до продуктивності та випадку використання:

1. **Найшвидший:** **Precalculated Pagination**:
   1. Так працює FastComments, коли ви використовуєте наші готові віджети та клієнти.
   2. Натискання «next» просто збільшує номер сторінки.
   3. Ви можете уявити це як отримання з сховища ключ‑значення.
   4. У цьому випадку просто визначте параметр `page`, починаючи з `0`, та напрямок сортування як `direction`.
   5. Розміри сторінок можна налаштувати за допомогою правил кастомізації.
2. **Найбільш гнучкий:** **Flexible Pagination**:
   1. У цьому випадку ви можете визначити власні параметри `limit` та `skip`. Не передавайте `page`.
   2. Сортування `direction` також підтримується.
   3. `limit` — це загальна кількість, яку повернути після застосування `skip`.
      - Приклад: встановіть `skip = 200, limit = 100`, коли `page size = 100` і `page = 2`.
   4. Дочірні коментарі все ще враховуються у пагінації. Ви можете обійти це, використовуючи параметр `asTree`.
      - Ви можете пагінувати дочірні елементи за допомогою `limitChildren` та `skipChildren`.
      - Ви можете обмежити глибину повернутих гілок за допомогою `maxTreeDepth`.

### Теми

1. При використанні `Precalculated Pagination` коментарі групуються за *сторінкою*, і коментарі в темах впливають на загальну сторінку.
   1. У цьому випадку теми можна визначити на клієнті за допомогою `parentId`.
   2. Наприклад, на сторінці з одним коментарем верхнього рівня та 29 відповідями, і встановивши `page=0` у API — ви отримаєте лише коментар верхнього рівня та 29 дочірніх.
2. При використанні `Flexible Pagination` ви можете визначити параметр `parentId`.
   1. Встановіть його в null, щоб отримати лише коментарі верхнього рівня.
   2. Потім, щоб переглянути теми, викличте API ще раз і передайте `parentId`.
   3. Типове рішення — зробити запит API для коментарів верхнього рівня, а потім паралельно виконати запити API, щоб отримати коментарі для дочірніх елементів кожного коментаря.
3. __НОВЕ З лютого 2023!__ Отримуйте у вигляді дерева, використовуючи `&asTree=true`.
   1. Ви можете уявити це як `Flexible Pagination as a Tree`.
   2. Тільки коментарі верхнього рівня враховуються у пагінації.
   3. Встановіть `parentId=null`, щоб розпочати дерево з кореня (ви повинні встановити `parentId`).
   4. Встановіть `skip` та `limit` для пагінації.
   5. Встановіть `asTree` у `true`.
   6. Вартість у кредитах збільшується в `2x`, оскільки наш бекенд повинен виконати значно більше роботи в цьому сценарії.
   7. Встановіть `maxTreeDepth`, `limitChildren` та `skipChildren` за потребою.

### Пояснення дерев

При використанні `asTree` може бути важко розібратись у пагінації. Ось зручна графіка:

<div class="screenshot white-bg">
    <div class="title">Діаграма пагінації дерева</div>
    <img class="screenshot-image" src="/images/fastcomments-comments-api-tree.png" alt="Діаграма пагінації дерева" />
</div>

### Отримання коментарів у контексті користувача

API `/comments` можна використовувати в двох контекстах, для різних випадків використання:

- Для повернення коментарів, відсортованих і позначених інформацією для створення вашого власного клієнта.
  - У цьому випадку визначте параметр запиту `contextUserId`.
- Для отримання коментарів з вашого бекенду для кастомних інтеграцій.
  - Платформа за замовчуванням буде використовувати це без `contextUserId`.

[inline-code-attrs-start title = 'Коментарі попередньо обчислена пагінація'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&page=0&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR'
[inline-code-end]

[inline-code-attrs-start title = 'Коментарі гнучка пагінація'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10'
[inline-code-end]

[inline-code-attrs-start title = 'Коментарі гнучка пагінація у контексті користувача'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Коментарі гнучка пагінація у контексті користувача лише для коментарів верхнього рівня'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null'
[inline-code-end]

### Отримати коментарі у вигляді дерева

Можливо отримати коментарі у вигляді дерева, при цьому пагінація враховує лише коментарі верхнього рівня.

[inline-code-attrs-start title = 'Коментарі у вигляді дерева у контексті користувача'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true'
[inline-code-end]

Хочете отримати лише коментарі верхнього рівня та їхніх безпосередніх дочірніх? Ось один спосіб:

[inline-code-attrs-start title = 'Коментарі у вигляді дерева з максимальною глибиною'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&urlId=test&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&maxTreeDepth=1&limitChildren=10'
[inline-code-end]

Однак у вашому інтерфейсі може знадобитися знати, чи показувати кнопку «показати відповіді» на кожному коментарі. При отриманні коментарів у вигляді дерева до коментарів додається властивість `hasChildren`, коли це застосовано.

### Отримати коментарі у вигляді дерева, пошук за хештегом

Можливо шукати за хештегом за допомогою API по всьому вашому орендарю (не обмежено однією сторінкою чи `urlId`).

У цьому прикладі ми опускаємо `urlId` і шукаємо за кількома хештегами. API поверне лише коментарі, які містять усі запитані хештеги.

[inline-code-attrs-start title = 'Коментарі у вигляді дерева у контексті користувача, за хештегом'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&direction=MR&skip=20&limit=10&contextUserId=my-user-id&parentId=null&asTree=true&hashTag=TestTag&hashTag=OtherTestTag'
[inline-code-end]

### Усі параметри запиту

[inline-code-attrs-start title = 'Структура запиту коментарів'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Ідентифікатор urlId (URL сторінки або ID статті), з яким пов'язані коментарі. **/
    urlId?: string
    /** Обмежити коментарі, повернуті цим користувачем. **/
    userId?: string
    /** Використовуйте це для пошуку за хештегом. Щоб отримати перетин кількох хештегів, використовуйте &hashTag=a&hashTag=b. **/
    hashTag?: string
    /** Напрямок сортування. За замовчуванням MR (Найбільш релевантний). Інші варіанти: OF (Спочатку найстаріші) та NF (Спочатку найновіші). **/
    direction?: 'MR' | 'OF' | 'NF'
    /** Попередньо обчислена пагінація: Сторінка для отримання, починаючи з 0. Передайте -1 для всіх коментарів (до 250). **/
    page?: number
    /** Гнучка пагінація: Скільки коментарів повернути? **/
    limit?: number
    /** Гнучка пагінація: Скільки дочірніх коментарів повернути для кожного батька? **/
    limitChildren?: number
    /** Гнучка пагінація: Скільки коментарів пропустити? **/
    skip?: number
    /** Гнучка пагінація: Скільки дочірніх коментарів пропустити для кожного батька? **/
    skipChildren?: number
    /** Для визначення заблокованих та позначених коментарів. **/
    contextUserId?: string
    /** Для визначення заблокованих та позначених коментарів. **/
    anonUserId?: string
    /** Для отримання дочірніх коментарів. **/
    parentId?: string
    /** Для отримання у вигляді дерева. **/
    asTree?: boolean
    /** Наскільки глибоко в дереві повертати дані? 0 — без дочірніх, 1 — лише безпосередні дочірні тощо. **/
    maxTreeDepth?: number
}
[inline-code-end]

### Відповідь

[inline-code-attrs-start title = 'Структура відповіді коментарів'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Включено у випадку помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-date' | 'unauthorized-page' | 'invalid-pagination-request' | 'invalid-limit' | 'invalid-limit-children' | 'invalid-skip' | 'invalid-skip-children' | 'invalid-max-tree-depth'
    /** Включено у випадку помилки. **/
    reason?: string
    /** Коментарі! **/
    comments: Comment[]
}
[inline-code-end]

### Корисні поради

#### URL ID

Ви, ймовірно, захочете використовувати API `Comment` з параметром `urlId`. Спочатку можна викликати API `Pages`, щоб побачити, які значення `urlId` доступні.

#### Анонімні дії

Для анонімних коментарів, ймовірно, варто передавати `anonUserId` під час отримання коментарів та під час позначення та блокування.

(!) Це вимога багатьох магазинів додатків, оскільки користувачі повинні мати можливість позначати створений користувачем контент, який вони бачать, навіть якщо вони не ввійшли в систему. Якщо цього не зробити, ваш додаток може бути видалений з цього магазину.

#### Коментарі не повертаються

Переконайтеся, що ваші коментарі схвалені і не є спамом.

---