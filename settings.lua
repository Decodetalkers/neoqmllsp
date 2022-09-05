local configs = require("lspconfig.configs")
local nvim_lsp = require("lspconfig")
if not configs.neoqml then
    configs.neoqml = {
        default_config = {
            --cmd = { "./target/debug/qmllsp" },
            cmd = { 'nc','localhost', '9257' },
            filetypes = { "qmljs" },
            root_dir = function(fname)
                return nvim_lsp.util.find_git_ancestor(fname)
            end,
            on_attach = function(client, bufnr)
                vim.notify("Lsp Start")
                require("cmps.cmp_onattach")(client, bufnr)
            end
        }
    }
    nvim_lsp.neoqml.setup({})
end
