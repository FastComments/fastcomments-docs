### Maven

Добавьте репозиторий Repsy в POM вашего проекта:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Затем добавьте необходимые зависимости:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.2</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.2</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.2</version>
    </dependency>
</dependencies>
```

### Gradle

Добавьте репозиторий Repsy в файл build.gradle:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:3.1.2"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:3.1.2"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:3.1.2"
}
```

### Состав библиотеки

Эта библиотека состоит из трёх модулей: сгенерированный API-клиент, основная Java-библиотека, содержащая написанные вручную утилиты
для упрощения работы с API, и модуль `pubsub` — библиотека для подписки на ленты изменений.

- [Документация API-клиента](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документация основной библиотеки, включая примеры SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документация библиотеки PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Публичные и защищённые API

В API-клиенте есть три класса: `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` содержит методы, требующие ваш API-ключ, а `PublicApi` содержит методы,
которые можно вызывать напрямую из браузера/мобильного устройства и т.д. без аутентификации.

`ModerationApi` предоставляет обширный набор быстрых API для модерации в реальном времени. Каждый метод `ModerationApi` принимает параметр `sso` и может выполнять аутентификацию через SSO или cookie сессии FastComments.com.