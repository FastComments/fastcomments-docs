### Maven

Додајте Repsy репозиторијум у POM вашег пројекта:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Затим додајте зависности које су вам потребне:

```xml
<dependencies>
    <!-- API клијент -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- Основна библиотека (укључује SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- PubSub библиотека (за догађаје уживо) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.2</version>
    </dependency>
</dependencies>
```

### Gradle

Додајте Repsy репозиторијум у ваш build.gradle фајл:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API клијент
    implementation "com.fastcomments:client:1.3.2"
    
    // Основна библиотека (укључује SSO)
    implementation "com.fastcomments:core:1.3.2"
    
    // PubSub библиотека (за догађаје уживо)
    implementation "com.fastcomments:pubsub:1.3.2"
}
```

### Садржај библиотеке

Ова библиотека садржи три модула. Генерисани API клијент, основна Java библиотека која садржи ручно написане алатке да би рад са API-јем био лакши, и `pubsub` модул који је библиотека за претплату на фидове промјена.

- [Документација API клијент библиотеке](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документација основне библиотеке, укључујући примјере SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документација PubSub библиотеке](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Јавни и заштићени API-ји

За API клијента постоје двије класе, `DefaultApi` и `PublicApi`. `DefaultApi` садржи методе које захтијевају ваш API кључ, а `PublicApi` садржи API позиве који се могу извршити директно из прегледача/мобилног уређаја/итд. без аутентификације.