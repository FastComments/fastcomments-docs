### Maven

Добавете Repsy хранилището към POM файла на вашия проект:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

След това добавете нужните зависимости:

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

Добавете Repsy хранилището към вашия build.gradle файл:

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

### Съдържание на библиотеката

Тази библиотека съдържа три модула: генерирания API клиент, основната Java библиотека, която съдържа ръчно написани помощни инструменти
за улесняване на работата с API-то, и модула `pubsub`, който е библиотека за абонамент за емисии с промени.

- [Документация на API клиентската библиотека](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документация на основната библиотека, включително примери за SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документация на PubSub библиотеката](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Публични срещу защитени API

За API клиента има три класа: `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` съдържа методи, които изискват вашия API ключ, а `PublicApi` съдържа методи,
които могат да се извикват директно от браузър/мобилно устройство и т.н. без автентикация.

`ModerationApi` предоставя обширен набор от API за бърза модерация в реално време. Всеки метод на `ModerationApi` приема параметър `sso` и може да се автентикира чрез SSO или чрез сесийна бисквитка на FastComments.com.