var sourcesIndex = JSON.parse('{\
"boa_ast":["",[["declaration",[],["export.rs","import.rs","mod.rs","variable.rs"]],["expression",[["literal",[],["array.rs","mod.rs","object.rs","template.rs"]],["operator",[["assign",[],["mod.rs","op.rs"]],["binary",[],["mod.rs","op.rs"]],["unary",[],["mod.rs","op.rs"]],["update",[],["mod.rs","op.rs"]]],["conditional.rs","mod.rs"]]],["access.rs","await.rs","call.rs","identifier.rs","mod.rs","new.rs","optional.rs","parenthesized.rs","spread.rs","tagged_template.rs","yield.rs"]],["function",[],["arrow_function.rs","async_arrow_function.rs","async_function.rs","async_generator.rs","class.rs","generator.rs","mod.rs","parameters.rs"]],["module_item_list",[],["mod.rs"]],["statement",[["iteration",[],["break.rs","continue.rs","do_while_loop.rs","for_in_loop.rs","for_loop.rs","for_of_loop.rs","mod.rs","while_loop.rs"]]],["block.rs","if.rs","labelled.rs","mod.rs","return.rs","switch.rs","throw.rs","try.rs","with.rs"]]],["keyword.rs","lib.rs","operations.rs","pattern.rs","position.rs","property.rs","punctuator.rs","statement_list.rs","visitor.rs"]],\
"boa_datagen":["",[],["datagen.rs"]],\
"boa_engine":["",[["builtins",[["array",[],["array_iterator.rs","mod.rs"]],["array_buffer",[],["mod.rs"]],["async_function",[],["mod.rs"]],["async_generator",[],["mod.rs"]],["async_generator_function",[],["mod.rs"]],["bigint",[],["mod.rs"]],["boolean",[],["mod.rs"]],["dataview",[],["mod.rs"]],["date",[],["mod.rs","utils.rs"]],["error",[],["aggregate.rs","eval.rs","mod.rs","range.rs","reference.rs","syntax.rs","type.rs","uri.rs"]],["escape",[],["mod.rs"]],["eval",[],["mod.rs"]],["function",[],["arguments.rs","mod.rs"]],["generator",[],["mod.rs"]],["generator_function",[],["mod.rs"]],["intl",[["collator",[],["mod.rs","options.rs"]],["list_format",[],["mod.rs","options.rs"]],["locale",[],["mod.rs","options.rs","utils.rs"]],["segmenter",[],["iterator.rs","mod.rs","options.rs","segments.rs"]]],["date_time_format.rs","mod.rs","options.rs"]],["iterable",[],["async_from_sync_iterator.rs","mod.rs"]],["json",[],["mod.rs"]],["map",[],["map_iterator.rs","mod.rs","ordered_map.rs"]],["math",[],["mod.rs"]],["number",[],["conversions.rs","globals.rs","mod.rs"]],["object",[],["for_in_iterator.rs","mod.rs"]],["promise",[],["mod.rs"]],["proxy",[],["mod.rs"]],["reflect",[],["mod.rs"]],["regexp",[],["mod.rs","regexp_string_iterator.rs"]],["set",[],["mod.rs","ordered_set.rs","set_iterator.rs"]],["string",[],["mod.rs","string_iterator.rs"]],["symbol",[],["mod.rs"]],["typed_array",[],["integer_indexed_object.rs","mod.rs"]],["uri",[],["consts.rs","mod.rs"]],["weak",[],["mod.rs","weak_ref.rs"]],["weak_map",[],["mod.rs"]],["weak_set",[],["mod.rs"]]],["mod.rs"]],["bytecompiler",[["declaration",[],["declaration_pattern.rs","mod.rs"]],["expression",[],["assign.rs","binary.rs","mod.rs","object_literal.rs","unary.rs","update.rs"]],["statement",[],["block.rs","break.rs","continue.rs","if.rs","labelled.rs","loop.rs","mod.rs","switch.rs","try.rs","with.rs"]]],["class.rs","env.rs","function.rs","jump_control.rs","mod.rs","module.rs","utils.rs"]],["context",[],["hooks.rs","icu.rs","intrinsics.rs","maybe_shared.rs","mod.rs"]],["environments",[],["compile.rs","mod.rs","runtime.rs"]],["object",[["builtins",[],["jsarray.rs","jsarraybuffer.rs","jsdataview.rs","jsdate.rs","jsfunction.rs","jsgenerator.rs","jsmap.rs","jsmap_iterator.rs","jspromise.rs","jsproxy.rs","jsregexp.rs","jsset.rs","jsset_iterator.rs","jstypedarray.rs","mod.rs"]],["internal_methods",[],["arguments.rs","array.rs","bound_function.rs","function.rs","integer_indexed.rs","mod.rs","proxy.rs","string.rs"]]],["jsobject.rs","mod.rs","operations.rs","property_map.rs"]],["optimizer",[["pass",[],["constant_folding.rs","mod.rs"]]],["mod.rs","walker.rs"]],["property",[["attribute",[],["mod.rs"]]],["mod.rs"]],["string",[],["common.rs","mod.rs"]],["value",[["conversions",[],["mod.rs","serde_json.rs","try_from_js.rs"]]],["display.rs","equality.rs","hash.rs","integer.rs","mod.rs","operations.rs","type.rs"]],["vm",[["call_frame",[],["abrupt_record.rs","env_stack.rs","mod.rs"]],["flowgraph",[],["color.rs","edge.rs","graph.rs","mod.rs","node.rs"]],["opcode",[["await_stm",[],["mod.rs"]],["binary_ops",[],["logical.rs","macro_defined.rs","mod.rs"]],["call",[],["mod.rs"]],["concat",[],["mod.rs"]],["control_flow",[],["break.rs","catch.rs","continue.rs","finally.rs","labelled.rs","mod.rs","return.rs","throw.rs","try.rs"]],["copy",[],["mod.rs"]],["define",[["class",[],["getter.rs","method.rs","mod.rs","setter.rs"]]],["mod.rs","own_property.rs"]],["delete",[],["mod.rs"]],["dup",[],["mod.rs"]],["environment",[],["mod.rs"]],["generator",[],["mod.rs","yield_stm.rs"]],["get",[],["function.rs","generator.rs","mod.rs","name.rs","private.rs","property.rs"]],["iteration",[],["for_in.rs","get.rs","iterator.rs","loop_ops.rs","mod.rs"]],["jump",[],["mod.rs"]],["new",[],["mod.rs"]],["nop",[],["mod.rs"]],["pop",[],["mod.rs"]],["push",[["class",[],["field.rs","mod.rs","private.rs"]]],["array.rs","environment.rs","literal.rs","mod.rs","new_target.rs","numbers.rs","object.rs"]],["require",[],["mod.rs"]],["rest_parameter",[],["mod.rs"]],["set",[],["class_prototype.rs","home_object.rs","mod.rs","name.rs","private.rs","property.rs","prototype.rs"]],["swap",[],["mod.rs"]],["switch",[],["mod.rs"]],["to",[],["mod.rs"]],["unary_ops",[],["decrement.rs","increment.rs","logical.rs","mod.rs","void.rs"]],["value",[],["mod.rs"]]],["mod.rs"]]],["code_block.rs","completion_record.rs","mod.rs"]]],["bigint.rs","class.rs","error.rs","job.rs","lib.rs","native_function.rs","realm.rs","symbol.rs","tagged.rs"]],\
"boa_gc":["",[["internals",[],["ephemeron_box.rs","gc_box.rs","mod.rs","weak_map_box.rs"]],["pointers",[],["ephemeron.rs","gc.rs","mod.rs","rootable.rs","weak.rs","weak_map.rs"]]],["cell.rs","lib.rs","trace.rs"]],\
"boa_icu_provider":["",[["data",[["min",[["normalizer",[["comp_v1",[],["mod.rs"]],["nfd_v1",[],["mod.rs"]],["nfdex_v1",[],["mod.rs"]],["nfkd_v1",[],["mod.rs"]],["nfkdex_v1",[],["mod.rs"]]],["mod.rs"]],["props",[["idc_v1",[],["mod.rs"]],["ids_v1",[],["mod.rs"]]],["mod.rs"]]],["mod.rs"]]]]],["lib.rs"]],\
"boa_interner":["",[],["fixed_string.rs","interned_str.rs","lib.rs","raw.rs","sym.rs"]],\
"boa_macros":["",[],["lib.rs"]],\
"boa_parser":["",[["lexer",[],["comment.rs","cursor.rs","error.rs","identifier.rs","mod.rs","number.rs","operator.rs","private_identifier.rs","regex.rs","spread.rs","string.rs","template.rs","token.rs"]],["parser",[["cursor",[["buffered_lexer",[],["mod.rs"]]],["mod.rs"]],["expression",[["assignment",[],["arrow_function.rs","async_arrow_function.rs","conditional.rs","exponentiation.rs","mod.rs","yield.rs"]],["left_hand_side",[["optional",[],["mod.rs"]]],["arguments.rs","call.rs","member.rs","mod.rs","template.rs"]],["primary",[["array_initializer",[],["mod.rs"]],["async_function_expression",[],["mod.rs"]],["async_generator_expression",[],["mod.rs"]],["class_expression",[],["mod.rs"]],["function_expression",[],["mod.rs"]],["generator_expression",[],["mod.rs"]],["object_initializer",[],["mod.rs"]],["template",[],["mod.rs"]]],["mod.rs"]]],["await_expr.rs","identifiers.rs","mod.rs","unary.rs","update.rs"]],["function",[],["mod.rs"]],["statement",[["block",[],["mod.rs"]],["break_stm",[],["mod.rs"]],["continue_stm",[],["mod.rs"]],["declaration",[["hoistable",[["async_function_decl",[],["mod.rs"]],["async_generator_decl",[],["mod.rs"]],["class_decl",[],["mod.rs"]],["function_decl",[],["mod.rs"]],["generator_decl",[],["mod.rs"]]],["mod.rs"]]],["export.rs","import.rs","lexical.rs","mod.rs"]],["expression",[],["mod.rs"]],["if_stm",[],["mod.rs"]],["iteration",[],["do_while_statement.rs","for_statement.rs","mod.rs","while_statement.rs"]],["labelled_stm",[],["mod.rs"]],["return_stm",[],["mod.rs"]],["switch",[],["mod.rs"]],["throw",[],["mod.rs"]],["try_stm",[],["catch.rs","finally.rs","mod.rs"]],["variable",[],["mod.rs"]],["with",[],["mod.rs"]]],["mod.rs"]]],["mod.rs"]]],["error.rs","lib.rs","source.rs"]],\
"boa_profiler":["",[],["lib.rs"]],\
"boa_runtime":["",[["console",[],["mod.rs"]]],["lib.rs"]],\
"boa_tester":["",[["exec",[],["js262.rs","mod.rs"]]],["edition.rs","main.rs","read.rs","results.rs"]],\
"boa_unicode":["",[],["lib.rs","tables.rs"]],\
"boa_wasm":["",[],["lib.rs"]]\
}');
createSourceSidebar();
