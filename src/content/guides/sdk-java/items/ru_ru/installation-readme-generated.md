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
    <!-- Клиент API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- Основная библиотека (включает SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- Библиотека PubSub (для живых событий) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.1</version>
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
    // Клиент API
    implementation "com.fastcomments:client:3.1.1"
    
    // Основная библиотека (включает SSO)
    implementation "com.fastcomments:core:3.1.1"
    
    // Библиотека PubSub (для живых событий)
    implementation "com.fastcomments:pubsub:3.1.1"
}
```

### Содержимое библиотеки

Эта библиотека содержит три модуля: сгенерированный клиент API, основную Java‑библиотеку, которая содержит написанные вручную утилиты для упрощения работы с API, и модуль `pubsub`, который представляет собой библиотеку для подписки на потоки изменений.

- [Документация библиотеки клиента API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документация основной библиотеки, включая примеры SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документация библиотеки PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Публичные и защищённые API

Для клиента API существует три класса, `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` содержит методы, требующие вашего API‑ключа, а `PublicApi` содержит методы, которые можно вызывать напрямую из браузера/мобильного устройства и т.д. без аутентификации.

`ModerationApi` предоставляет обширный набор живых и быстрых API модерации. Каждый метод `ModerationApi` принимает параметр `sso` и может аутентифицироваться через SSO или сессионный cookie FastComments.com.