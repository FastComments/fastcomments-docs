Tilføj FastComments SDK til din apps `build.gradle.kts`-fil:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:0.0.1")
}
```

Sørg for, at du har Repsy-repositoriet i dit projekts `settings.gradle.kts`:

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