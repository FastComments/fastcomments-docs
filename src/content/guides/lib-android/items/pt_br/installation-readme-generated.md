Adicione o SDK do FastComments ao arquivo `build.gradle.kts` do seu aplicativo:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:2.0.0")
}
```

Certifique-se de que você tem o repositório Repsy no `settings.gradle.kts` do seu projeto:

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