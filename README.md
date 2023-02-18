# qml lsp based on Tower and treesitter


It is a qml lsp based on tower-lsp and treesitter a tool 



## Setup

```lua
local configs = require("lspconfig.configs")
local nvim_lsp = require("lspconfig")
if not configs.neoqml then
    configs.neoqml = {
        default_config = {
            cmd = { "neoqmllsp" },
            filetypes = { "qmljs" },
            root_dir = function(fname)
                return nvim_lsp.util.find_git_ancestor(fname)
            end,
            single_file_support = true,-- suggested
            on_attach = on_attach
        }
    }
    nvim_lsp.neocmake.setup({})
end
```


## Features

* complete
* symbol\_provider

## TODO
* Undefined function check

