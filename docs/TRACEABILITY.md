# TRACEABILITY

| Q | D | Жизненный цикл | Задачи | Реализация |
|---|---|---|---|---|
| [Q1](questions/Q1.md) | [D15](decisions/D15-evolution-credo2.md) | in work | [T-10](tasks/T-10-workspace-phase-0/README.md) ⬜ | [`features/README.md`](features/README.md) (шапка) |
| [Q2](questions/Q2.md) | [D16](decisions/D16-dsl-canon-regex-mvp.md) | open | — | [`parser.feature`](features/parser.feature), [`lexer.feature`](features/lexer.feature) (целевое v0.2) |
| [Q3](questions/Q3.md) | [D16](decisions/D16-dsl-canon-regex-mvp.md) | open | — | [`parser.feature`](features/parser.feature), [`lexer.feature`](features/lexer.feature) (целевое v0.2) |
| [Q4](questions/Q4.md) | [D17](decisions/D17-priority-out-of-mvp.md) | open | — | [`parser.feature`](features/parser.feature), [`execution.feature`](features/execution.feature), [`editor.feature`](features/editor.feature), [`lsp.feature`](features/lsp.feature), [`client_explanation.feature`](features/client_explanation.feature) (целевое v0.2) |
| [Q5](questions/Q5.md) | [D19](decisions/D19-statuses-priorities-canon.md) | done | — | [`features/README.md`](features/README.md) (статусы, приоритеты, счётчики) |
| [Q6](questions/Q6.md) | [D19](decisions/D19-statuses-priorities-canon.md) | done | — | [`features/README.md`](features/README.md) (счётчики; проверка — [`../tests/features_inventory.rs`](../tests/features_inventory.rs)) |
| [Q7](questions/Q7.md) | [D52](decisions/D52-glossary-terms-canon.md) | in work | [T-07](tasks/T-07-meta-fields/README.md) ⬜ | — |
| [Q8](questions/Q8.md) | [D21](decisions/D21-core-semantics-v01.md) | done | — | [`errors.feature`](features/errors.feature), [`explain_full.feature`](features/explain_full.feature) |
| [Q9](questions/Q9.md) | [D21](decisions/D21-core-semantics-v01.md) | done | — | [`errors.feature`](features/errors.feature) |
| [Q10](questions/Q10.md) | [D21](decisions/D21-core-semantics-v01.md) | done | — | [`execution.feature`](features/execution.feature), [`test_draft.feature`](features/test_draft.feature) |
| [Q11](questions/Q11.md) | [D53](decisions/D53-error-messages-language.md) | done | — | [`evaluate.feature`](features/evaluate.feature), [`rest_api.feature`](features/rest_api.feature), [`rest_auth.feature`](features/rest_auth.feature), [`errors.feature`](features/errors.feature) |
| [Q12](questions/Q12.md) | [D54](decisions/D54-source-of-truth-flow.md) | in work | [T-16](tasks/T-16-stale-check-test/README.md) ✅, [T-08](tasks/T-08-materialize-source-file/README.md) ⬜, [T-01](tasks/T-01-draft-source-hash/README.md) ✅ | [`draft.feature`](features/draft.feature), [`test_draft.feature`](features/test_draft.feature), [`publish.feature`](features/publish.feature), [`notebook_ui.feature`](features/notebook_ui.feature), [`editor.feature`](features/editor.feature), [`file_management.feature`](features/file_management.feature) |
| [Q13](questions/Q13.md) | [D14](decisions/D14-published-artifact-canon.md) | in work | [T-06](tasks/T-06-registry-path-xyz/README.md) ⬜, [T-07](tasks/T-07-meta-fields/README.md) ⬜ | [`publish.feature`](features/publish.feature), [`storage_paths.feature`](features/storage_paths.feature), [`publish_rules.feature`](features/publish_rules.feature), [`immutability.feature`](features/immutability.feature), [`mcp_tools.feature`](features/mcp_tools.feature) |
| [Q14](questions/Q14.md) | [D55](decisions/D55-publish-branch-name.md) | done | — | [`publish.feature`](features/publish.feature), [`publish_rules.feature`](features/publish_rules.feature) |
| [Q15](questions/Q15.md) | [D56](decisions/D56-merge-step.md) | in work | [T-17](tasks/T-17-merge-command/README.md) ⬜ | [`git_integration.feature`](features/git_integration.feature), [`publish_rules.feature`](features/publish_rules.feature), [`deferred.feature`](features/deferred.feature) |
| [Q16](questions/Q16.md) | [D32](decisions/D32-test-gate-mvp.md) | in work | [T-02](tasks/T-02-test-gate/README.md) ⬜ | [`publish.feature`](features/publish.feature), [`test_draft.feature`](features/test_draft.feature) |
| [Q17](questions/Q17.md) | [D57](decisions/D57-bare-git-immutability.md) | in work | [T-06](tasks/T-06-registry-path-xyz/README.md) ⬜, [T-17](tasks/T-17-merge-command/README.md) ⬜ | [`immutability.feature`](features/immutability.feature) |
| [Q18](questions/Q18.md) | [D35](decisions/D35-semver-v01.md) | done | — | [`semver.feature`](features/semver.feature), [`deferred.feature`](features/deferred.feature), [`storage_paths.feature`](features/storage_paths.feature) |
| [Q19](questions/Q19.md) | [D58](decisions/D58-workspace-data-dirs.md) | open | — | [`notebook_ui.feature`](features/notebook_ui.feature) |
| [Q20](questions/Q20.md) | [D22](decisions/D22-rest-paths-canon.md) | done | — | [`rest_api.feature`](features/rest_api.feature), [`evaluate.feature`](features/evaluate.feature), [`batch.feature`](features/batch.feature), [`import_export.feature`](features/import_export.feature) |
| [Q21](questions/Q21.md) | [D23](decisions/D23-get-checks-manifest.md) | done | — | [`manifest.feature`](features/manifest.feature), [`rest_api.feature`](features/rest_api.feature), [`evaluate.feature`](features/evaluate.feature), [`dashboard.feature`](features/dashboard.feature) |
| [Q22](questions/Q22.md) | [D26](decisions/D26-rest-auth-x-api-key.md) | done | [T-24](tasks/T-24-rest-cli-contour-test/README.md) ✅ | [`rest_auth.feature`](features/rest_auth.feature), [`evaluate.feature`](features/evaluate.feature) |
| [Q23](questions/Q23.md) | [D25](decisions/D25-rest-error-envelope.md) | done | — | [`rest_api.feature`](features/rest_api.feature), [`evaluate.feature`](features/evaluate.feature), [`rest_auth.feature`](features/rest_auth.feature), [`errors.feature`](features/errors.feature) |
| [Q24](questions/Q24.md) | [D36](decisions/D36-batch-deferred.md) | open | — | [`batch.feature`](features/batch.feature) (целевое v0.2) |
| [Q25](questions/Q25.md) | [D37](decisions/D37-client-explanation-deferred.md) | open | — | [`client_explanation.feature`](features/client_explanation.feature) (целевое v0.2) |
| [Q26](questions/Q26.md) | [D24](decisions/D24-import-export-deferred.md) | open | — | [`import_export.feature`](features/import_export.feature) (целевое v0.2) |
| [Q27](questions/Q27.md) | [D27](decisions/D27-rest-launch-address.md) | done | — | — |
| [Q28](questions/Q28.md) | [D31](decisions/D31-check-create-contract.md) | done | [T-03](tasks/T-03-check-create/README.md) ✅ | [`draft.feature`](features/draft.feature), [`agent_minimal.feature`](features/agent_minimal.feature) |
| [Q29](questions/Q29.md) | [D34](decisions/D34-mcp-tool-contracts.md) | in work | [T-04](tasks/T-04-mcp-errors/README.md) ✅, [T-05](tasks/T-05-mcp-success-schemas/README.md) ⬜, [T-21](tasks/T-21-mcp-test-struct-api/README.md) ✅, [T-22](tasks/T-22-mcp-draft-test-fix/README.md) ✅ | [`mcp_tools.feature`](features/mcp_tools.feature), [`draft.feature`](features/draft.feature), [`test_draft.feature`](features/test_draft.feature), [`publish.feature`](features/publish.feature), [`deprecation.feature`](features/deprecation.feature), [`manifest_sync.feature`](features/manifest_sync.feature) |
| [Q30](questions/Q30.md) | [D29](decisions/D29-notebook-mcp-transport.md) | open | — | [`agent_minimal.feature`](features/agent_minimal.feature) |
| [Q31](questions/Q31.md) | [D12](decisions/D12-agent-chat-panel.md) | open | — | [`notebook_ui.feature`](features/notebook_ui.feature), [`agent_minimal.feature`](features/agent_minimal.feature), [`inline_execution.feature`](features/inline_execution.feature) |
| [Q32](questions/Q32.md) | [D28](decisions/D28-two-git-contours.md) | in work | [T-06](tasks/T-06-registry-path-xyz/README.md) ⬜ | [`storage_paths.feature`](features/storage_paths.feature), [`git_integration.feature`](features/git_integration.feature), [`agent_minimal.feature`](features/agent_minimal.feature), [`publish.feature`](features/publish.feature), [`publish_rules.feature`](features/publish_rules.feature), [`immutability.feature`](features/immutability.feature), [`deprecation.feature`](features/deprecation.feature), [`mcp_tools.feature`](features/mcp_tools.feature), [`manifest_sync.feature`](features/manifest_sync.feature) |
| [Q33](questions/Q33.md) | [D30](decisions/D30-execution-mechanism.md) | in work | [T-08](tasks/T-08-materialize-source-file/README.md) ⬜, [T-09](tasks/T-09-check-run/README.md) ⬜ | [`inline_execution.feature`](features/inline_execution.feature), [`agent_minimal.feature`](features/agent_minimal.feature), [`draft.feature`](features/draft.feature), [`mcp_tools.feature`](features/mcp_tools.feature) |
| [Q34](questions/Q34.md) | [D32](decisions/D32-test-gate-mvp.md) | in work | [T-02](tasks/T-02-test-gate/README.md) ⬜ | [`inline_execution.feature`](features/inline_execution.feature), [`notebook_ui.feature`](features/notebook_ui.feature), [`publish.feature`](features/publish.feature), [`test_draft.feature`](features/test_draft.feature) |
| [Q35](questions/Q35.md) | [D59](decisions/D59-workspace-templates.md) | open | — | [`notebook_ui.feature`](features/notebook_ui.feature), [`file_management.feature`](features/file_management.feature) |
| [Q36](questions/Q36.md) | [D18](decisions/D18-pipelines-out-lsp-mvp.md) | open | — | [`lsp.feature`](features/lsp.feature), [`graph_view.feature`](features/graph_view.feature) |
| [Q37](questions/Q37.md) | [D33](decisions/D33-fields-registry-source.md) | open | — | [`lsp.feature`](features/lsp.feature), [`notebook_ui.feature`](features/notebook_ui.feature) |
| [Q38](questions/Q38.md) | [D18](decisions/D18-pipelines-out-lsp-mvp.md) | open | — | [`lsp.feature`](features/lsp.feature) |
| [Q39](questions/Q39.md) | [D6](decisions/D6-lsp-degradation.md) | open | — | [`lsp_notebook.feature`](features/lsp_notebook.feature) |
| [Q40](questions/Q40.md) | [D20](decisions/D20-features-docs-dod.md) | done | — | [`features/testing.feature`](features/testing.feature), [`features/README.md`](features/README.md) (нота Q5/Q40), [`../../tests/features_inventory.rs`](../../tests/features_inventory.rs), `.github/workflows/ci.yml` |
| [Q41](questions/Q41.md) | [D60](decisions/D60-docs-ownership-sync.md) | done | — | [`features/README.md`](features/README.md) |
| [Q42](questions/Q42.md) | [D21](decisions/D21-core-semantics-v01.md) | done | — | [`explain.feature`](features/explain.feature), [`explain_full.feature`](features/explain_full.feature) |
| [Q43](questions/Q43.md) | [D38](decisions/D38-agent-cycle.md) | done | [T-11](tasks/T-11-agent-cycle/README.md) ✅ | [`agents-cycle.feature`](features/agents-cycle.feature), [`agents-rework.feature`](features/agents-rework.feature), [`agents-memory-mail.feature`](features/agents-memory-mail.feature), [`agents-sized-routes.feature`](features/agents-sized-routes.feature), [`agents-git-approval.feature`](features/agents-git-approval.feature), [`agents-audit.feature`](features/agents-audit.feature) |
| [Q44](questions/Q44.md) | [D39](decisions/D39-loop-dispatcher.md) | done | [T-12](tasks/T-12-agent-loop/README.md) ✅ | [`agents-cycle.feature`](features/agents-cycle.feature), [`agents-rework.feature`](features/agents-rework.feature), [`agents-memory-mail.feature`](features/agents-memory-mail.feature), [`agents-sized-routes.feature`](features/agents-sized-routes.feature), [`agents-git-approval.feature`](features/agents-git-approval.feature), [`agents-audit.feature`](features/agents-audit.feature) |
| [Q45](questions/Q45.md) | [D40](decisions/D40-scope-threshold.md) | done | [T-13](tasks/T-13-agent-hardening/README.md) ✅ | — |
| [Q46](questions/Q46.md) | [D41](decisions/D41-dispatch-refinements.md) | in work | [T-13](tasks/T-13-agent-hardening/README.md) ✅, [T-14](tasks/T-14-grammar-message-sync/README.md) ⬜ | — |
| [Q47](questions/Q47.md) | [D42](decisions/D42-expect-iteration.md) | done | — | — |
| [Q48](questions/Q48.md) | [D43](decisions/D43-auditor-mail.md) | done | — | — |
| [Q49](questions/Q49.md) | [D44](decisions/D44-run5-refinements.md) | done | — | — |
| [Q50](questions/Q50.md) | [D45](decisions/D45-wave0-quality-config.md) | in work | [T-15](tasks/T-15-mcp-ready-process/README.md) 🚧 | — |
| [Q51](questions/Q51.md) | [D46](decisions/D46-product-process-commits.md) | in work | [T-15](tasks/T-15-mcp-ready-process/README.md) 🚧 | — |
| [Q52](questions/Q52.md) | [D47](decisions/D47-git-refinements-run5.md) | done | — | — |
| [Q53](questions/Q53.md) | [D48](decisions/D48-findings-registry-owner.md) | done | — | `analysis/findings-registry.md` |
| [Q54](questions/Q54.md) | [D49](decisions/D49-validator-branch-contains.md) | done | — | — |
| [Q55](questions/Q55.md) | [D50](decisions/D50-dod-by-package-scope.md) | done | — | — |
| [Q56](questions/Q56.md) | [D51](decisions/D51-agent-tools-token-hygiene.md) | done | — | `.opencode/scripts/agents-perms.mjs` (новый), канон ролей |
| [Q57](questions/Q57.md) | [D61](decisions/D61-archive-removal.md) | done | — | — |
| [Q58](questions/Q58.md) | [D62](decisions/D62-brief-journal-rules.md) | done | — | — |
| [Q59](questions/Q59.md) | [D63](decisions/D63-journal-index-lifecycle.md) | done | — | — |
| [Q60](questions/Q60.md) | [D64](decisions/D64-journal-integrity-test.md) | done | [T-18](tasks/T-18-docs-journal-test/README.md) ✅ | — |
| [Q61](questions/Q61.md) | [D65](decisions/D65-reference-policy.md) | done | [T-25](tasks/T-25-d65-analysis-addresses/README.md) ✅ | — |
| [Q62](questions/Q62.md) | [D66](decisions/D66-doc-quality-checks.md) | in work | [T-19](tasks/T-19-doc-quality-checks/README.md) ⬜ | — |
| [Q63](questions/Q63.md) | [D67](decisions/D67-cspell-deferred.md) | open | — | — |
| [Q64](questions/Q64.md) | [D68](decisions/D68-changelog-handwritten.md) | done | — | — |
| [Q65](questions/Q65.md) | [D69](decisions/D69-retro-decisions.md) | open | — | [`wasm.feature`](features/wasm.feature) (ретро-линия D11, v0.2) |
| [Q66](questions/Q66.md) | [D70](decisions/D70-spec-reduction.md) | done | — | — |
| [Q67](questions/Q67.md) | [D71](decisions/D71-journal-rules-relocation.md) | done | — | — |
| [Q68](questions/Q68.md) | [D72](decisions/D72-changelog-full-cleanup.md) | done | — | — |
| [Q69](questions/Q69.md) | [D73](decisions/D73-readme-entrypoints.md) | done | — | — |
| [Q70](questions/Q70.md) | [D74](decisions/D74-grammar-normative-focus.md) | done | — | — |
| [Q71](questions/Q71.md) | [D75](decisions/D75-git-lean-workflow.md) | done | — | — |
| [Q72](questions/Q72.md) | [D76](decisions/D76-traceability-links-only.md) | done | — | — |
| [Q73](questions/Q73.md) | [D77](decisions/D77-tasks-visibility-completeness.md) | done | [T-18](tasks/T-18-docs-journal-test/README.md) ✅ | — |
| [Q74](questions/Q74.md) | [D78](decisions/D78-t15-mcp-ready-program.md) | in work | [T-15](tasks/T-15-mcp-ready-process/README.md) 🚧 | [`agents-state-schema.feature`](features/agents-state-schema.feature), [`agents-session-checkpoint.feature`](features/agents-session-checkpoint.feature), [`agents-re-raise.feature`](features/agents-re-raise.feature), [`agents-metrics.feature`](features/agents-metrics.feature), [`agents-mcp-readiness.feature`](features/agents-mcp-readiness.feature) |
| [Q75](questions/Q75.md) | [D79](decisions/D79-journal-canon-completeness.md) | done | — | — |
| [Q76](questions/Q76.md) | [D80](decisions/D80-features-visibility-completeness.md) | done | [T-18](tasks/T-18-docs-journal-test/README.md) ✅ | — |
| [Q77](questions/Q77.md) | [D81](decisions/D81-pm-process-mining.md) | done | — | — |
| [Q78](questions/Q78.md) | [D82](decisions/D82-traceability-lifecycle-waves.md) | done | [T-20](tasks/T-20-traceability-wave2/README.md) ✅ | — |
| [Q79](questions/Q79.md) | [D83](decisions/D83-traceability-wave2.md) | done | [T-20](tasks/T-20-traceability-wave2/README.md) ✅ | — |
| [Q80](questions/Q80.md) | [D84](decisions/D84-rules-revision.md) | done | — | `.opencode/rules/**`, `AGENTS.md` §«Служебная зона и аудит» (волна 1) |
| [Q81](questions/Q81.md) | — | open | — | — |
| [Q82](questions/Q82.md) | [D85](decisions/D85-sverka-snapshot-scope.md) | done | — | — |
| [Q83](questions/Q83.md) | [D86](decisions/D86-state-schema.md) | in work | [T-15](tasks/T-15-mcp-ready-process/README.md) 🚧 | `.opencode/rules/state-schema.md` (новый канон), `dispatch-loop.md`, `AGENTS.md` (карта) |
| [Q84](questions/Q84.md) | [D87](decisions/D87-t24-pilot-fixes.md) | in work | [T-15](tasks/T-15-mcp-ready-process/README.md) 🚧 | `.opencode/agents/{lead,git,analyst,tester}.md`, `.opencode/rules/{review,git-workflow,dispatch-loop}.md` |
| [Q85](questions/Q85.md) | [D88](decisions/D88-c9-c12-loop-tuning.md) | in work | [T-15](tasks/T-15-mcp-ready-process/README.md) 🚧 | `.opencode/agents/{lead,analyst}.md`, `.opencode/rules/dispatch-loop.md`, `AGENTS.md` (§«Лимиты шагов», §«Память и почта») |
| [Q86](questions/Q86.md) | [D89](decisions/D89-branch-topology-freeze-session-commit.md) | in work | [T-15](tasks/T-15-mcp-ready-process/README.md) 🚧 | `.opencode/rules/{git-workflow,dispatch-loop}.md`, `.opencode/agents/{analyst,lead}.md`, `.opencode/rules/review.md` |
| [Q87](questions/Q87.md) | [D90](decisions/D90-c5-c7-re-raise-selfreport.md) | in work | [T-15](tasks/T-15-mcp-ready-process/README.md) 🚧 | `.opencode/rules/{dispatch-loop,state-schema}.md`, `.opencode/agents/{analyst,lead}.md`, [`agents-re-raise.feature`](features/agents-re-raise.feature), [`agents-state-schema.feature`](features/agents-state-schema.feature), [`agents-session-checkpoint.feature`](features/agents-session-checkpoint.feature) |
| [Q88](questions/Q88.md) | [D91](decisions/D91-c2-validate-state.md) | in work | [T-15](tasks/T-15-mcp-ready-process/README.md) 🚧 | `.opencode/scripts/validate-state.mjs`, `.opencode/rules/{state-schema,dispatch-loop}.md`, `.opencode/agents/validator.md`, `.opencode/rules/review.md`, `AGENTS.md` |
| [Q89](questions/Q89.md) | [D92](decisions/D92-credited-run-predicate.md) | in work | [T-15](tasks/T-15-mcp-ready-process/README.md) 🚧 | `.opencode/rules/state-schema.md` (раздел «Зачётный прогон») |
| [Q90](questions/Q90.md) | [D93](decisions/D93-session-commit-process-branch.md) | in work | [T-15](tasks/T-15-mcp-ready-process/README.md) 🚧 | `.opencode/rules/{git-workflow,dispatch-loop}.md`, `.opencode/commands/git/{checkpoint,status}.md`, `.opencode/agents/{git,auditor}.md`, `.opencode/scripts/session-checkpoint.mjs`, `AGENTS.md`, [`agents-session-checkpoint.feature`](features/agents-session-checkpoint.feature) |
| [Q91](questions/Q91.md) | [D94](decisions/D94-b2-profile-flag.md) | in work | [T-15](tasks/T-15-mcp-ready-process/README.md) 🚧 | `.opencode/plugins/token-guard.ts` (B2_PROFILE), `.opencode/scripts/token-guard-test.mjs` (новый), `AGENTS.md` (§«Гигиена и чтение») |
| [Q92](questions/Q92.md) | [D95](decisions/D95-phase-e-mcp-design.md) | in work | [T-26](tasks/T-26-mcp-server-design/README.md) ⬜ | [`agents-mcp-readiness.feature`](features/agents-mcp-readiness.feature) |
| [Q93](questions/Q93.md) | [D96](decisions/D96-state-metrics.md) | in work | [T-15](tasks/T-15-mcp-ready-process/README.md) 🚧 | `.opencode/scripts/state-metrics.mjs` (новый), `.opencode/rules/dispatch-loop.md`, `AGENTS.md`, [`agents-metrics.feature`](features/agents-metrics.feature) |

Легенда жизненного цикла: `open` — есть вопрос (решения нет либо решение есть, но
требуется задача) · `in work` — есть открытая задача · `done` — закрыто (вопрос
закрыт / задача закрыта). `resolved` упразднён ([Q78](questions/Q78.md)/[D82](decisions/D82-traceability-lifecycle-waves.md));
`dropped` в таблице не используется — снятое показывается `done`. В колонке
«Задачи» — `T-XX` со статусом из [`tasks/`](tasks/README.md) (⬜/🚧/✅) либо `—`;
«Реализация» — требования, статусы — в [`features/README.md`](features/README.md).
Колонки Q и D — только ссылки на записи журнала; темы — в
[`questions/README.md`](questions/README.md) и
[`decisions/README.md`](decisions/README.md) (D63, D76).
