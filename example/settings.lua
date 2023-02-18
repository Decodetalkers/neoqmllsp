--local configs = require("lspconfig.configs")
local nvim_lsp = require("lspconfig")

local root_files = { 'build' }
return {
    lsp = {
        neoqml = {
            default_config = {
                --cmd = { "./target/debug/qmllsp" },
                cmd = vim.lsp.rpc.connect('127.0.0.1', '9257'),
                filetypes = { "qmljs" },
                root_dir = function(fname)
                    return nvim_lsp.util.root_pattern(unpack(root_files))(fname)
                end,
                on_attach = function(client, bufnr)
                    vim.notify("Lsp Start")
                    require("cmps.cmp_onattach")(client, bufnr)
                end
            }
        }
    }
}
