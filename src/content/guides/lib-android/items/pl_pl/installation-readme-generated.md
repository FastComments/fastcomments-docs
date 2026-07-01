---
Dodaj SDK FastComments do pliku `build.gradle.kts` swojej aplikacji:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:2.0.0")
}
```

Upewnij się, że masz repozytorium Repsy w pliku `settings.gradle.kts` swojego projektu:

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
---