#### Sakai

Sakai поддържа LTI 1.3 Dynamic Registration в версии с LTI Advantage. От Administration Workspace:

1. Влезте като администратор на Sakai и отворете **Administration Workspace**.
2. Изберете **External Tools** > **Install LTI 1.3 Tool**.
3. Поставете URL адреса за регистрация на FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">вземете го оттук</a>) и изпратете.
4. Одобрете инструмента, когато handshake процесът завърши.

След това инструментът се появява под **External Tools** и може да бъде добавян към сайтовете от техните поддържащи.

#### Schoology

Инстанциите на Schoology Enterprise поддържат LTI 1.3, но наличието на Dynamic Registration варира според разполагането. Проверете с вашия мениджър на акаунта в Schoology.

Ако Dynamic Registration не е наличен във вашата инстанция на Schoology, ще трябва да конфигурирате интеграцията ръчно, като използвате следните endpoints:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

След като Schoology ви даде Client ID и Deployment ID, свържете се с поддръжката на FastComments, за да регистрираме конфигурацията във вашия tenant.

#### Други LTI 1.3 платформи

Всяка LMS, която следва спецификацията IMS LTI 1.3 Advantage, трябва да работи със същия URL за регистрация (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">вземете го оттук</a>). Потърсете настройка, означена като "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" или подобно.

Ако вашата платформа поддържа само ръчна настройка на LTI 1.3, използвайте четирите endpoints, изброени в раздела за Schoology по-горе, и се свържете с поддръжката за финализиране.