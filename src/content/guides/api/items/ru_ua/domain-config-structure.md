A `DomainConfig` object представляет конфигурацию для домена для арендатора.

The structure for the `DomainConfig` object is as follows:

[inline-code-attrs-start title = 'Структура DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Домен, а не URL, например "fastcomments.com" или "www.example.com". Можно включить поддомен, если нужно ограничить до поддомена. Максимум 1000 символов. **/
    domain: string
    /** Имя отправителя (From-Name), используемое при отправке писем. **/
    emailFromName?: string
    /** Адрес отправителя (From-Email), используемый при отправке писем. Убедитесь, что SPF настроен так, чтобы mail.fastcomments.com мог отправлять письма от имени домена, указанный в этом атрибуте. **/
    emailFromEmail?: string
    /** ТОЛЬКО ДЛЯ ЧТЕНИЯ. Когда объект был создан. **/
    createdAt: string
    /** Логотип, связанный с этим доменом. Используется в письмах. Используйте HTTPS. **/
    logoSrc?: string
    /** Меньший логотип, связанный с этим доменом. Используйте HTTPS. **/
    logoSrc100px?: string
    /** ТОЛЬКО ДЛЯ SSO. URL, используемый в футере каждого отправляемого письма. Поддерживает переменную "[userId]". **/
    footerUnsubscribeURL?: string
    /** ТОЛЬКО ДЛЯ SSO. Заголовки, используемые в каждом отправляемом письме. Полезно, например, для установки заголовков, связанных с отпиской, чтобы улучшить доставляемость. Запись List-Unsubscribe в этом Record, если она существует, поддерживает переменную "[userId]". **/
    emailHeaders?: Record<string, string>
    /** Отключить все ссылки для отписки. Не рекомендуется, может ухудшить показатели доставляемости. **/
    disableUnsubscribeLinks?: boolean
    /** Конфигурация DKIM. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура конфигурации DKIM'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** Имя домена в вашей DKIM-записи. **/
    domainName: string
    /** Селектор ключа DKIM, который нужно использовать. **/
    keySelector: string
    /** Ваш приватный ключ. Начинается с -----BEGIN PRIVATE KEY----- и заканчивается -----END PRIVATE KEY----- **/
    privateKey: string
}
[inline-code-end]

### Для аутентификации

Конфигурация домена используется для определения сайтов, которые могут размещать виджет FastComments для вашей учетной записи. Это базовый вид аутентификации, поэтому добавление или удаление любой конфигурации домена может повлиять на доступность вашей установки FastComments в продакшене.

Не удаляйте и не изменяйте свойство `domain` у `Domain Config` для домена, который в настоящее время используется, если только вы не намерены отключить этот домен.

Это имеет тот же эффект, что и удаление домена через [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Также обратите внимание, что удаление домена из интерфейса `My Domains` удалит любую соответствующую конфигурацию для этого домена, которая могла быть добавлена через этот интерфейс.

### Для настройки писем

Ссылку для отписки в футере письма и функцию отписки в один клик, предлагаемую многими почтовыми клиентами, можно настроить через этот API, задав соответственно `footerUnsubscribeURL` и `emailHeaders`.

### Для DKIM

После определения DKIM записей DNS просто обновите DomainConfig своей конфигурацией DKIM, используя определённую структуру. 

---