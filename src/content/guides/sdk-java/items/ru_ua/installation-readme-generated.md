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
        <version>1.3.1</version>
    </dependency>
    
    <!-- Основная библиотека (включает SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- Библиотека PubSub (для событий в реальном времени) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.1</version>
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
    implementation "com.fastcomments:client:1.3.1"
    
    // Основная библиотека (включает SSO)
    implementation "com.fastcomments:core:1.3.1"
    
    // Библиотека PubSub (для событий в реальном времени)
    implementation "com.fastcomments:pubsub:1.3.1"
}
```

### Library Contents

Эта библиотека содержит три модуля. Сгенерированный клиент API, основная Java-библиотека, содержащая написанные вручную утилиты для облегчения работы с API, и модуль `pubsub`, который представляет собой библиотеку для подписки на потоки изменений.

- [Документация клиента API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документация основной библиотеки, включая примеры SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документация библиотеки PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Для клиента API существуют два класса: `DefaultApi` и `PublicApi`. `DefaultApi` содержит методы, которые требуют вашего API-ключа, а `PublicApi` содержит вызовы API, которые можно выполнять непосредственно из браузера/мобильного устройства/и т. д. без аутентификации.