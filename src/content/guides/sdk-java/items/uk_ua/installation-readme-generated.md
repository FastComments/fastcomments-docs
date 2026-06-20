### Maven

Додайте репозиторій Repsy у POM вашого проекту:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Потім додайте потрібні залежності:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Додайте репозиторій Repsy у файл build.gradle:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:2.0.0"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:2.0.0"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### Вміст бібліотеки

Ця бібліотека містить три модулі. Згенерований клієнт API, базова Java-бібліотека, яка містить вручну написані утиліти для спрощення роботи з API, та модуль `pubsub`, який є бібліотекою для підписки на стрічки змін.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Публічні та захищені API

Для клієнта API існують три класи: `DefaultApi`, `PublicApi` і `ModerationApi`. `DefaultApi` містить методи, які вимагають вашого API-ключа, а `PublicApi` містить методи, які можна викликати безпосередньо з браузера/мобільного пристрою тощо без автентифікації.

`ModerationApi` забезпечує роботу панелі модерації. Він містить методи для модерації коментарів (перелік, підрахунок, пошук, журнали та експорт), дії модерації (видалення/відновлення, позначення, встановлення статусу на перегляд/спам/підтвердження, голоси та повторне відкриття/закриття треду), банів (заборона коментувати, скасування бану, підсумки перед баном, статус бану та налаштування, а також підрахунок заблокованих користувачів) та значків і довіри (нагородження/видалення значка, ручні значки, отримання/встановлення коефіцієнта довіри та внутрішній профіль користувача). Кожен метод `ModerationApi` приймає параметр `sso`, щоб виклик можна було виконати від імені модератора, автентифікованого через SSO.