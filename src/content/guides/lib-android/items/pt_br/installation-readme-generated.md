Adicione o SDK do FastComments ao arquivo `build.gradle.kts` do seu app:

```kotlin
dependencies {
    implementation("com.fastcomments:android-sdk:2.1.0")
}
```

Certifique-se de ter o repositório Repsy no `settings.gradle.kts` do seu projeto:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // outros repositórios...
    }
}
```