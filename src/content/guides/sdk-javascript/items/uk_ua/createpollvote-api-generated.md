Проголосуйте в опитуванні або перемістіть наявний голос до іншого варіанта. Учасник має максимум один голос на
опитування, тому повторний виклик для того самого учасника переміщує його голос, а не додає новий.

Це дотримується налаштувань опитувань сайту: якщо голосування дозволено лише авторизованим користувачам, голос із лише
anonUserId буде відхилено, а анонімні голоси обмежуються за частотою на IP на опитування.

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| createPollVoteBody | CreatePollVoteBody | Так |  |

## Відповідь

Повертає: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreatePollVoteResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад createPollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";

  const vote: CreatePollVoteBody = {
    pollId: "poll-2024-09",
    optionId: "option-A"
    // userId є необов'язковим і тут пропущений
  };

  const result: CreatePollVoteResponse = await createPollVote(tenantId, vote);
  console.log(result);
})();
[inline-code-end]