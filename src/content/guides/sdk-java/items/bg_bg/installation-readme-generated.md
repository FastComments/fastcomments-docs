### Maven

Добавете репозитория Repsy към POM файла на вашия проект:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

След това добавете зависимостите, които са ви необходими:

```xml
<dependencies>
    <!-- API клиент -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- Основна библиотека (включва SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- PubSub библиотека (за събития в реално време) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.2</version>
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
    // API клиент
    implementation "com.fastcomments:client:1.3.2"
    
    // Основна библиотека (включва SSO)
    implementation "com.fastcomments:core:1.3.2"
    
    // PubSub библиотека (за събития в реално време)
    implementation "com.fastcomments:pubsub:1.3.2"
}
```

### Съдържание на библиотеката

Тази библиотека съдържа три модула. Генерираният API клиент, основната Java библиотека, която съдържа ръчно написани помощни средства, за да улесни работата с API-то, и модулът `pubsub`, който е библиотека за абониране за потоци от промени.

- [Документация за библиотеката на API клиента](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документация на основната библиотека, включително примери за SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документация за PubSub библиотеката](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Публични срещу защитени API-та

За API клиента има два класа, `DefaultApi` и `PublicApi`. `DefaultApi` съдържа методи, които изискват вашия API ключ, а `PublicApi` съдържа API повиквания, които могат да се извършват директно от браузър/мобилно устройство/и т.н. без удостоверяване.