use serde::{Deserialize, Serialize};

macro_rules! id_newtype {
    ($name:ident) => {
        #[derive(
            Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(pub i64);

        impl From<i64> for $name {
            fn from(value: i64) -> Self {
                Self(value)
            }
        }

        impl From<$name> for i64 {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

id_newtype!(UserId);
id_newtype!(RepoId);

id_newtype!(BranchId);
id_newtype!(ProtectedBranchId);
id_newtype!(ProtectedTagId);

id_newtype!(CommitStatusId);
