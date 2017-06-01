use corollary_support::*;

use Data::Foldable;
use Data::List;
use Data::Map;
use Data::Maybe;

#[derive(Debug, Eq, Ord)]
pub enum ItemKind {
    Enum,
    Struct,
    Union,
    Type,
    Symbol
}
pub use self::ItemKind::*;

pub fn mergeCrateMaps() -> Map::Map<String, CrateMap> {
    Map::fromListWith((Map::unionWith((__op_addadd))))
}

pub fn parseCrateMap() -> Either<String, CrateMap> {
    fmap(root, foldrM(parseLine, (Map::empty, vec![]), filter((not(null)), map(cleanLine, lines))))
}

pub fn rewritesFromCratesMap(crates: CratesMap) -> ItemRewrites {
    Map::fromList(/* Expr::Generator */ Generator)
}

pub fn splitModuleMap(modName: String, crates: CratesMap) -> (ModuleMap, CratesMap) {
    fromMaybe((vec![], crates))(/* do */ {
        let thisCrate = Map::lookup("".to_string(), crates);

        let thisModule = Map::lookup(modName, thisCrate);

        let thisCrate_q = Map::delete(modName, thisCrate);

        let crates_q = Map::insert("".to_string(), thisCrate_q, crates);

        (thisModule, crates_q)
    })
}



