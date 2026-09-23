A `Poll` прикріплюється до коментаря, а не є окремим об’єктом. Він створюється разом з коментарем  
(див. `POST /api/v1/comments`), або додається до існуючого коментаря пізніше за допомогою `PUT /api/v1/polls/:commentId`.

Кількість голосів зберігається безпосередньо в опитуванні, тому читання опитування дає вам результати без необхідності підсумовувати їх. Окремі голоси, що стоять за цими підрахунками, є об’єктами `PollVote`.

Кожен варіант має `id`, який генерується під час створення опитування. Цей id використовується для подачі голосу, зміни мітки варіанту та збереження варіанту (разом з його голосами) під час `PUT` опитування з доданими або видаленими варіантами. Це єдиний безпечний спосіб посилання на варіант — ніколи не використовуйте його позицію у списку.

[inline-code-attrs-start title = 'Структура опитування'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPollOption {
    id: string
    label: string
    votes: number
}

interface CommentPoll {
    question: string
    options: CommentPollOption[]
    totalVotes: number
    /** When set and in the past, the poll is closed and no longer accepts votes. **/
    closesAt?: string | null
    /** 0 anonymous (the default), 1 admins and moderators, 2 everyone. Absent means anonymous. **/
    privacy?: 0 | 1 | 2 | null
    /** When true, the counts are hidden from anyone who has not voted yet. Absent means false. **/
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

### Обмеження

- Питання є обов’язковим і має максимум 200 символів.  
- Опитування має від 2 до 10 варіантів.  
- Мітка варіанту є обов’язковою, має максимум 100 символів і повинна бути унікальною в межах опитування (без урахування регістру).  
- `closesAt` має бути в майбутньому під час створення опитування. Щоб закрити опитування негайно, виконайте `PATCH` з датою в минулому.

### Налаштування сайту

Опитування підкоряються конфігурації вашого сайту, яку можна змінити під **Customize Widget**:

- Опитування мають бути ввімкнені, перш ніж можна створити опитування, інакше API відповість `polls-disabled`.  
- Голосування може бути обмежене лише зареєстрованими користувачами; у цьому випадку голос, надісланий лише з `anonUserId`, буде відхилений з помилкою `poll-login-required`.