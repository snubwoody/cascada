use crate::Layout;

pub trait DebugTree: Layout {
    fn print_children(&self, indent: usize) {
        for node in self.children() {
            node.print(indent);
            node.print_children(indent + 1);
        }
    }

    fn print(&self, indent: usize) {
        let size = self.size();
        let position = self.position();
        let label = self.label();
        let whitespace = " ".repeat(indent);
        println!("{whitespace}•{label}(size: {size}, position: {position})");
    }

    // TODO: only expose one method
    fn debug_tree(&self) {
        self.print(0);
        self.print_children(1);
    }
}

impl<L: Layout> DebugTree for L {}

impl DebugTree for dyn Layout {}
