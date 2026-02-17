A `DomainConfig` object represents configuration for a domain for a tenant.

The structure for the `DomainConfig` object is as follows:

[inline-code-attrs-start title = 'Структура конфигурации домена'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Домен, не URL, например "fastcomments.com" или "www.example.com". Поддомен может быть включён, если нужно ограничить до поддомена. Максимум 1000 символов. **/
    domain: string
    /** Имя отправителя, используемое при отправке писем. **/
    emailFromName?: string
    /** Адрес электронной почты отправителя, используемый при отправке писем. Убедитесь, что SPF настроен, чтобы разрешить mail.fastcomments.com отправлять письма от домена, указанного в этом поле. **/
    emailFromEmail?: string
    /** ТОЛЬКО ДЛЯ ЧТЕНИЯ. Когда объект был создан. **/
    createdAt: string
    /** Логотип, связанный с этим доменом. Используется в письмах. Используйте HTTPS. **/
    logoSrc?: string
    /** Меньший логотип, связанный с этим доменом. Используйте HTTPS. **/
    logoSrc100px?: string
    /** Только для SSO. URL, используемый в подвале каждого отправляемого письма. Поддерживает переменную "[userId]". **/
    footerUnsubscribeURL?: string
    /** Только для SSO. Заголовки, используемые в каждом отправляемом письме. Полезно, например, для установки заголовков, связанных с отпиской, чтобы улучшить доставляемость. Запись List-Unsubscribe в этой записи, если она существует, поддерживает переменную "[userId]". **/
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
    /** Селектор ключа DKIM для использования. **/
    keySelector: string
    /** Публичный ключ в формате PEM. Возвращается в ответах на GET. **/
    publicKey: string
    /** @deprecated Больше не возвращается в ответах API. Принимается при записи для обратной совместимости. **/
    privateKey?: string
}
[inline-code-end]

### Для аутентификации

Конфигурация домена используется для определения, какие сайты могут размещать виджет FastComments для вашей учётной записи. Это базовая форма
аутентификации, означающая, что добавление или удаление любых конфигураций доменов может повлиять на доступность вашей установки FastComments
в продакшене.

Не удаляйте и не изменяйте свойство `domain` объекта `Domain Config` для домена, который в настоящее время используется, если только вы не намерены отключить этот домен.

Это имеет такое же поведение, как удаление домена из [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Также обратите внимание, что удаление домена из интерфейса `My Domains` удалит любую соответствующую конфигурацию для этого домена, которая могла быть добавлена через этот интерфейс.

### Для настройки электронной почты

Ссылку для отписки в подвале письма и функцию отписки в один клик, предлагаемую многими почтовыми клиентами, можно настроить через этот API, определив соответственно `footerUnsubscribeURL` и `emailHeaders`.

### Для DKIM

После определения DKIM-записей DNS просто обновите DomainConfig с вашей конфигурацией DKIM, используя указанную структуру.

---