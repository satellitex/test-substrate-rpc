// fromMetadata (metadata: Metadata): ModulesWithMethods {
//  return metadata.asV4.modules
//    .filter((modul) => modul.calls.isSome)
//    .reduce((result, modul: ModuleMetadata, sectionIndex) => {
//      const section = stringCamelCase(modul.name.toString());
//
//      result[section] = modul.calls.unwrap().reduce((newModule, callMetadata, methodIndex) => {
//        const method = stringCamelCase(callMetadata.name.toString());
//
//        newModule[method] = createUnchecked(section, sectionIndex, methodIndex, callMetadata);
//
//        return newModule;
//      }, {} as Methods);
//
//      return result;
//    }, { ...extrinsics });
//}

//
//export interface MethodFunction {
//  (...args: any[]): Method;
//  callIndex: Uint8Array;
//  meta: FunctionMetadataV4;
//  method: string;
//  section: string;
//  toJSON: () => any;
//}
//
//export interface Methods {
//  [key: string]: MethodFunction;
//}
//
//export interface ModulesWithMethods {
//  [key: string]: Methods; // Will hold modules returned by state_getMetadata
//}

//pub struct Method {
//
//}
//
//pub struct MethodFunction {
//    method: Fn<Args...> -> Method;
//    meta: FunctionMetadataV4;
//    method: String;
//    section: String;
//    toJSON: Fn<()> -> String;
//}
//
//
//pub struct Methods {
//    method_function: HashMap<String, MethodFunction>
//}
//
//pub struct ModuleWithMethods {
//    methods: HashMap<String, Methods>;
//}
//
//pub fn from_metadata(metadata: Metadata) -> ModuleWithMethods {
//
//}