Если ваш val уже знает, кто посетитель, Secure SSO передаёт эту идентификацию виджету, чтобы они никогда не видели второй вход. Не требуется создавать конечные точки и ничего вызывать во время выполнения: вы вычисляете три значения на сервере и передаёте их в конфигурацию виджета.

Val Town поставляет вход без конфигурации с `std/oauth`, поэтому посетитель может войти с учётной записью Val Town, которую он уже имеет. Замените это на то, что использует ваше приложение; часть FastComments не меняется.

## Создание полезной нагрузки на сервере

Секрет API подписывает полезную нагрузку и никогда не должен попадать в код браузера. Установите SDK из npm, который работает в среде Deno от Val Town без изменений:

[inline-code-attrs-start title = 'sso.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { SecureSSOPayloadBuilder } from "npm:fastcomments-sdk/server";

export function buildSSOPayload(user) {
  // id must be stable for the same person, or they get a new comment identity on every login.
  const id = `vt-${user.id}`;

  return new SecureSSOPayloadBuilder(Deno.env.get("FASTCOMMENTS_API_SECRET"), {
    id,
    // email is required and must be unique.
    email: user.email ?? `${id}@users.noreply.val.town`,
    // username is required and cannot be an email.
    username: user.username ?? id,
    displayName: user.username ?? undefined,
    avatar: user.links.profileImageUrl ?? undefined,
  }).getPayload();
}
[inline-code-end]

`getPayload()` возвращает `{ userDataJSONBase64, verificationHash, timestamp }`. Эти три значения — всё, что попадает в браузер. Секрет подписывает их и затем отбрасывается, поэтому ничего на странице не позволяет читателю подделать другого пользователя.

## Передача в виджет

[inline-code-attrs-start title = 'Конфигурация виджета с SSO'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { getOAuthUserData, oauthMiddleware } from "https://esm.town/v/std/oauth/middleware.ts";

app.get("/", async (c) => {
  const session = await getOAuthUserData(c.req.raw);
  const user = session?.user;

  const config = {
    tenantId: TENANT_ID,
    urlId: "my-thread",
    ...(user
      ? { sso: { ...buildSSOPayload(user), logoutURL: "/logout" } }
      : { sso: { loginURL: "/auth/login" } }),
  };

  // ...render the widget with this config
});

export default oauthMiddleware(app.fetch);
[inline-code-end]

`oauthMiddleware` добавляет `GET /auth/login`, `GET /auth/callback` и `POST /auth/logout` за вас. Обратите внимание, что выход (logout) — это **POST**, в то время как виджет переходит к `logoutURL` с помощью GET, поэтому укажите `logoutURL` на небольшой маршрут вашего приложения, который выполняет POST.

Когда посетитель вышел из системы, передайте `sso` только с `loginURL`. Виджет тогда покажет запрос входа вместо анонимного поля комментария.

## Возможные проблемы

`timestamp` — это эпоха в **миллисекундах**, не должна быть в будущем и не должна быть старше двух дней. Генерируйте её на сервере в том же запросе, где вычисляется хеш. Генерация в браузере — классическая ошибка: значение отличается от того, которое было захешировано, и каждый комментарий отклоняется.

Никогда не устанавливайте `isAdmin` или `isModerator` из провайдера идентификации. Вход с учётной записью Val Town ничего не говорит о том, кто должен модерировать ваш сайт.

Смотрите [руководство по SSO](/guide-sso.html) для полного списка полей, потоков с групповыми ограничениями и значков.