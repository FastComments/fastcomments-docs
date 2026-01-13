Об'єкт `EmailTemplate` представляє конфігурацію для користувацького шаблону електронного листа для тенанта.

Система обиратиме шаблон електронного листа для використання за допомогою:

- Ідентифікатора типу, який ми називаємо `emailTemplateId`. Це константи.
- `domain`. Спочатку ми намагатимемося знайти шаблон для домену, до якого прив'язаний пов'язаний об'єкт (наприклад, `Comment`), і якщо відповідність не знайдена, тоді ми спробуємо знайти шаблон, де domain дорівнює null або `*`.

Структура об'єкта `EmailTemplate` виглядає так:

[inline-code-attrs-start title = 'Структура шаблону електронного листа'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplate {
    id: string
    tenantId: string
    emailTemplateId: string
    displayName: string
    /** ТІЛЬКИ ДЛЯ ЧИТАННЯ **/
    createdAt: string
    /** ТІЛЬКИ ДЛЯ ЧИТАННЯ **/
    updatedAt: string
    /** ТІЛЬКИ ДЛЯ ЧИТАННЯ **/
    updatedByUserId: string
    /** Домен, з яким має бути пов’язаний шаблон. **/
    domain?: string | '*' | null
    /** Вміст шаблону електронного листа в синтаксисі EJS. **/
    ejs: string
    /** Мапа перевизначених ключів перекладу на значення для кожної підтримуваної локалі. **/
    translationOverridesByLocale: Record<string, Record<string, string>>
    /** Об'єкт, який представляє контекст рендерингу шаблону. **/
    testData: object
}
[inline-code-end]

### Примітки

- Дійсні значення `emailTemplateId` можна отримати з кінцевої точки `/definitions`.
- Кінцева точка `/definitions` також містить стандартні переклади та тестові дані.
- Шаблони не збережуться, якщо структура або тестові дані недійсні.