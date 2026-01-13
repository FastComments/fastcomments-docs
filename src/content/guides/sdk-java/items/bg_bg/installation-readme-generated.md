### Maven

Добавете репозитория Repsy към POM-а на вашия проект:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

След това добавете необходимите зависимости:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>0.0.2</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>0.0.2</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>0.0.2</version>
    </dependency>
</dependencies>
```

### Gradle

Добавете репозитория Repsy във файла build.gradle:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:0.0.2"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:0.0.2"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:0.0.2"
}
```

### Library Contents

Тази библиотека съдържа три модула. Генерираният API клиент, основната Java библиотека, която съдържа ръчно написани помощни средства за улесняване на работата с API-то, и модула `pubsub`, който е библиотека за абониране за потоци от промени.

- [Документация на библиотеката за API клиент](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документация на Core библиотеката, включително примери за SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документация на PubSub библиотеката](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

За API клиента има два класа, `DefaultApi` и `PublicApi`. `DefaultApi` съдържа методи, които изискват вашия API ключ, а `PublicApi` съдържа API извиквания, които могат да бъдат направени директно от браузър/мобилно устройство/и т.н. без удостоверяване.