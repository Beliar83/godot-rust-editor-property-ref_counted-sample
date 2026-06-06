Steps:
- Build rust library in ./rust
- Open the Godot Editor from a shell to see the output
- Open the project in Godot
- Unfold the texture in the inspector and see the engine crash
- Notice that the output will contain something similar to this, indicating that the reference count will decrease over subsequent calls, while it should stay constant:
```
Instance: -9223370387805367753, Reference Count: 9
Instance: -9223370387805367753, Reference Count: 8
Instance: -9223370387805367753, Reference Count: 7
Instance: -9223370387805367753, Reference Count: 6
```
