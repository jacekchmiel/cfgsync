# cfgsync
Easy to setup config file synchronization based on git.

## Dependencies
cfgsync currently relies on git executable being present on path. Should work as
long as you are able to run git in your terminal.

## Getting it to work
Your need to initialize cfgsync on a machine you want to synchronize configs
from. 

At minimum you'll need a git repository with read/write access without the
interactive credentials prompt. You can use either ssh access or git built-in
password cache/storage.

Install cfgsync (Details TBD)

Configure cfgsync. You can commandline interface for that - see `cfgsync
configure --help`. At minimum you'll need to configure the `git_repo_url` only.
You can also configure `git_branch` (`main` by default) and `git_local_repo` aka
local clone path (`$HOME/.cfgsync.git` by default).

Select files you want to synchronize using `cfgsync add`

Make your first push with `cfgsync push`.

After these steps you can easily restore your configuration on any other machine
(or the same machine but with new os install if you like reinstalling your
system from scratch). You can do so using `cfgsync pull`.