# libcst_helper.py
import libcst as cst
from typing import Tuple

class PrependCommentTransformer(cst.CSTTransformer):
    def __init__(self, comment_text: str = "# Auto-prepended comment"):
        self.comment_text = comment_text

    def leave_FunctionDef(
        self,
        original_node: cst.FunctionDef,
        updated_node: cst.FunctionDef
    ) -> cst.FunctionDef:
        # Make a new EmptyLine with the comment and prepend it to leading_lines
        new_leading = (cst.EmptyLine(comment=cst.Comment(self.comment_text)),) + tuple(original_node.leading_lines)
        return updated_node.with_changes(leading_lines=new_leading)

def transform_code(source: str, comment_text: str = "# Auto-prepended comment") -> str:
    """
    Parse source, run transformer that prepends `comment_text` before each function,
    and return the transformed (round-tripped) source code.
    """
    module = cst.parse_module(source)
    transformer = PrependCommentTransformer(comment_text=comment_text)
    new_module = module.visit(transformer)
    return new_module.code
