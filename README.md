# keep-synced
A language-agnostic development tool that synchronizes text across multiple files.


## Overview

Say you're developing a Python project whose version you want to maintain in its `pyproject.toml` file:

```toml
[project]
version = "2020.0.0"
```

But you also want this same version to be available via the `__version__` attribute of the package in its `__init__.py` file:

```python
__version__ = "2020.0.0"
```

You might write a comment above both locations to let other developers know they need to update the version in the other file when they change it, but it could be easy to miss.

So you might even write a script (or use a tool) that specifically automates updating versions in multiple files, such as https://github.com/callowayproject/bump-my-version.

`keep-synced` is a tool that synchronizes *arbitrary* text across multiple language-agnostic files.

To use it, you add a comment above the "source of truth" line that describes how to capture the text in a regular expression named capture group:

```toml
[project]
# keep-synced /"(?<PROJECT_VERSION>.+)"/
version = "2020.0.0"
```

And then in the other file(s) that should stay in sync with the source of truth, you add a similar comment above the line(s) you want to synchronize that references the named capture group defined by the source of truth comment above:

```python
# keep-synced /"${PROJECT_VERSION}"/
__version__ = "2020.0.0"
```

Now let's say someone updated one instance of the version and forgot to update the other:

```diff
[project]
# keep-synced /"(?<PROJECT_VERSION>.+)"/
- version = "2020.0.0"
+ version = "2025.0.0"
```

Just run `keep-synced`, and it automatically updates the other location to match the source of truth!

```bash
> keep-synced

Updated 1 location.
```

And we can see that the other file was indeed synchronized with the change to our source of truth:

```diff
# keep-synced /"${PROJECT_VERSION}"/
- __version__ = "2020.0.0"
+ __version__ = "2025.0.0"
```

> [!NOTE]
> The same regex pattern used to define a named capture group at its source of truth is used to match on the text that should be replaced in other files that reference the named capture group in a `keep-synced` comment.
