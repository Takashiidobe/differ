# Differ

This binary takes one argument, a file path to the file you want to listen to changes for.

Afterwards, every one second it receives changes, it'll write a diff file to your current directory containing the changes of the last 1 second. If you make a change you want to undo but can't for some reason, you can use the diff files this utility creates to recover a file to a good state.

You can take these diffs and run `patch base.diff 1.diff` to apply the first diff, continuing onto applying the diffs until you reach the state you want (generally before the unwanted change). With that, you can recover the old file.
