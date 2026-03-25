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

След това добавете зависимостите, от които се нуждаете:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.1</version>
    </dependency>
</dependencies>
```

### Gradle

Добавете хранилището Repsy във файла build.gradle:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:1.3.1"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:1.3.1"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:1.3.1"
}
```

### Library Contents

Тази библиотека съдържа три модула. Генерираният API клиент, основната Java библиотека, която съдържа ръчно написани помощни средства за улесняване на работата с API-то, и модулът `pubsub`, който е библиотека за абониране за фийдове с промени.

- [Документация на API клиентската библиотека](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документация на основната библиотека, включително примери за SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документация на PubSub библиотеката](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

За API клиента има два класа, `DefaultApi` и `PublicApi`. `DefaultApi` съдържа методи, които изискват вашия API ключ, а `PublicApi` съдържа API повиквания, които могат да се извършват директно от браузър/мобилно устройство/и т.н. без удостоверяване.