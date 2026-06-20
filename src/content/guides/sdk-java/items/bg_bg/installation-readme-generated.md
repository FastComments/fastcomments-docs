### Maven

Добавете хранилището Repsy към POM-а на вашия проект:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

След това добавете зависимостите, от които имате нужда:

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

Добавете хранилището Repsy във вашия файл build.gradle:

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

### Съдържание на библиотеката

Тази библиотека съдържа три модула. Генерираният API клиент, основната Java библиотека, която съдържа ръчно написани помощни средства за улесняване на работата с API, и модулът `pubsub`, който е библиотека за абониране за потоци с промени.

- [Документация на API клиент библиотеката](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документация на Core библиотеката, включително примери за SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документация на PubSub библиотеката](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Публични и защитени API

За API клиента има три класа, `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` съдържа методи, които изискват вашия API ключ, а `PublicApi` съдържа методи, които могат да се извикват директно от браузър/мобилно устройство/и т.н. без удостоверяване.

`ModerationApi` захранва таблото на модератора. Той съдържа методи за модериране на коментари (списък, брой, търсене, логове и експорт), модераторски действия (премахване/възстановяване, маркиране, задаване на статус преглед/спам/одобрение, гласове и повторно отваряне/затваряне на тема), забрани (блокиране от коментиране, отмяна на бан, обобщения преди бан, статус на бана и предпочитания, и брой блокирани потребители) и значки & доверие (присъждане/премахване на значка, ръчни значки, получаване/задаване на фактор на доверие и вътрешен профил на потребителя). Всеки метод на `ModerationApi` приема параметър `sso`, за да може повикването да се извърши от името на модератор, удостоверен чрез SSO.