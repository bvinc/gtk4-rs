// Take a look at the license at the top of the repository in the LICENSE file.

//! Traits intended for blanket imports.

pub use crate::auto::traits::*;

pub use crate::actionable::ActionableExtManual;
pub use crate::builder::BuilderExtManual;
pub use crate::color_chooser::ColorChooserExtManual;
pub use crate::combo_box::ComboBoxExtManual;
pub use crate::dialog::DialogExtManual;
pub use crate::drawing_area::DrawingAreaExtManual;
pub use crate::editable::EditableExtManual;
pub use crate::entry::EntryExtManual;
pub use crate::entry_buffer::EntryBufferExtManual;
pub use crate::entry_completion::EntryCompletionExtManual;
pub use crate::flow_box::FlowBoxExtManual;
pub use crate::im_context_simple::IMContextSimpleExtManual;
pub use crate::list_box::ListBoxExtManual;
pub use crate::list_store::GtkListStoreExtManual;
pub use crate::notebook::NotebookExtManual;
pub use crate::overlay::OverlayExtManual;
pub use crate::spin_button::SpinButtonExtManual;
pub use crate::text_buffer::TextBufferExtManual;
pub use crate::tree_sortable::TreeSortableExtManual;
pub use crate::tree_store::TreeStoreExtManual;
pub use crate::widget::{InitializingWidgetExt, WidgetExtManual};

#[doc(hidden)]
pub use gdk::prelude::*;
#[doc(hidden)]
pub use gdk_pixbuf::prelude::*;
#[doc(hidden)]
pub use gio::prelude::*;
#[doc(hidden)]
pub use glib::prelude::*;
#[doc(hidden)]
pub use graphene::prelude::*;
#[doc(hidden)]
pub use gsk::prelude::*;
#[doc(hidden)]
pub use pango::prelude::*;
