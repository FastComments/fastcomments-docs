Adicione o FastComments SDK ao arquivo `build.gradle.kts` do seu aplicativo:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:0.0.1")
}
```

Certifique-se de que o repositório Repsy esteja no arquivo `settings.gradle.kts` do seu projeto:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // other repositories...
    }
}
```